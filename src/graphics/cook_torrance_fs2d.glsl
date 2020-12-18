// #extension GL_ARB_shading_language_include : require
// #include brdf_cook_torrance

uniform vec3 cam_pos;

//Input from previous stage
in vec3 v_color;
in vec2 v_uv;
in vec3 v_normal;
in vec3 v_wpos;
in vec4 v_tangent;

//Output for current stage
out vec3 frag_color;

#define PI 3.14159265359

struct Material {
    vec3 albedo;
    float roughness;
    float metallic;
};

struct Light {
    vec3 color;
    vec3 wpos;
    float power;
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

vec3 BRDF(Light light, vec3 F0, vec3 N, vec3 V, Material mat) {
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

vec3 ReinhardTonemap(vec3 col) {
    col = col / (col + vec3(1.0));
    return pow(col, vec3(1.0/2.2));
}

void main() {
    // frag_color = vec3(v_uv, 0.0);
    // frag_color = v_normal;

    vec3 v_binormal = cross(v_normal, v_tangent.xyz) * v_tangent.w;

    Material mat;
    mat.albedo = vec3(1.0, 0.0, 0.0);
    mat.metallic = 0.0;
    // mat.subsurf = 0.0;
    // mat.spec = 0.5;
    mat.roughness = 0.1;
    // mat.sheen = 0.0;
    // mat.clearcoat = 1.0;
    // mat.clearcoatGloss = 1.0;

    Light light;
    light.wpos = vec3(0.0); //Not important right now
    light.color = vec3(1.0);
    light.power = 3.0;

    vec3 F0 = vec3(0.04);
    F0 = mix(F0, mat.albedo, mat.metallic);

    vec3 N = normalize(v_normal);
    vec3 V = normalize(cam_pos - v_wpos);

    vec3 col = vec3(0.0);
    col += BRDF(light, F0, N, -V, mat);

    frag_color = ReinhardTonemap(col);
    // frag_color = col;
}
