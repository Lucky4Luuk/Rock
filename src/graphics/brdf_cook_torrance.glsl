#extension GL_ARB_shading_language_include : require

struct Material {
    vec3 albedo;
    float roughness;
    float metallic;
};

float DistributionGGX(vec3 N, vec3 H, float roughness) {
    float a      = roughness*roughness;
    float a2     = a*a;
    float NdotH  = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float num   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

float GeometrySchlickGGX(float NdotV, float roughness) {
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float num   = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return num / denom;
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness) {
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2  = GeometrySchlickGGX(NdotV, roughness);
    float ggx1  = GeometrySchlickGGX(NdotL, roughness);

    return ggx1 * ggx2;
}

vec3 fresnelSchlick(float cosTheta, vec3 F0) {
    return F0 + (1.0 - F0) * pow(max(1.0 - cosTheta, 0.0), 5.0);
}

vec3 Cook_Torrance_BRDF(Light light, vec3 F0, vec3 N, vec3 V, Material mat, vec3 v_wpos) {
    // lighting
    vec3 L = normalize(vec3(-0.55, 0.5, 0.55));
    // float dist = length(light.wpos - v_wpos);
    // float atten = 1.0 / (dist * dist);
    vec3 radiance = light.color * light.power; //multiply by atten for point lights
    vec3 H = normalize(V + L); //Half vector

    // cook-torrance brdf
    float k = mat.roughness;
    float NDF = DistributionGGX(N, H, k); //Normal distribution function
    float G   = GeometrySmith(N, V, L, k);
    vec3 F    = fresnelSchlick(max(dot(H, V), 0.0), F0);

    vec3 kS = F;
    vec3 kD = vec3(1.0) - kS;
    kD *= 1.0 - mat.metallic;

    vec3 numerator    = NDF * G * F;
    float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0);
    vec3 specular     = numerator / max(denominator, 0.001);

    float NdotL = max(dot(N, L), 0.0);
    return (kD * mat.albedo / PI + specular) * radiance * NdotL;
}
