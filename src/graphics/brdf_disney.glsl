//Broken

struct Material {
    vec3 albedo;
    float metallic;
    float subsurf;
    float spec; //TODO: Replace with IOR
    float roughness;
    float sheen;
    float clearcoat;
    float clearcoatGloss;
};

float SchlickFresnel(float u)
{
    float m = clamp(1-u, 0, 1);
    float m2 = m*m;
    return m2*m2*m; // pow(m,5)
}

float GTR1(float NdotH, float a)
{
    if (a >= 1) return 1/PI;
    float a2 = a*a;
    float t = 1 + (a2-1)*NdotH*NdotH;
    return (a2-1) / (PI*log(a2)*t);
}

float GTR2(float NdotH, float a)
{
    float a2 = a*a;
    float t = 1 + (a2-1)*NdotH*NdotH;
    return a2 / (PI * t*t);
}

float sqr(float x) { return x*x; }
float GTR2_aniso(float NdotH, float HdotX, float HdotY, float ax, float ay)
{
    return 1 / (PI * ax*ay * sqr( sqr(HdotX/ax) + sqr(HdotY/ay) + NdotH*NdotH ));
}

float smithG_GGX(float NdotV, float alphaG)
{
    float a = alphaG*alphaG;
    float b = NdotV*NdotV;
    return 1 / (NdotV + sqrt(a + b - a*b));
}

float smithG_GGX_aniso(float NdotV, float VdotX, float VdotY, float ax, float ay)
{
    return 1 / (NdotV + sqrt( (VdotX*ax)*(VdotX*ax) + (VdotY*ay)*(VdotY*ay) + NdotV*NdotV ));
}

vec3 BRDF(Light light, vec3 F0, vec3 N, vec3 V, vec3 X, vec3 Y, Material mat) {
    float roughness = mat.roughness;
    float roughsqr = roughness*roughness;

    vec3 L = normalize(vec3(-0.55, 0.5, 0.55));

    float NdotL = dot(N,L);
    float NdotV = dot(N,V);
    // if (NdotL < 0 || NdotV < 0) return vec3(0);

    vec3 H = normalize(L+V);
    float NdotH = dot(N,H);
    float LdotH = dot(L,H);

    //Luminance approximation
    vec3 Cdlin = mat.albedo; //pow(mat.albedo, vec3(2.2));
    float Cdlum = 0.3*Cdlin.x + 0.6*Cdlin.y + 0.1*Cdlin.z;

    //Normalize luminance to isolate hue and saturation
    vec3 Ctint = Cdlum > 0.0 ? Cdlin/Cdlum : vec3(1.0);
    //0.0 is specularTint in the original implementation, original range [0; 1]
    vec3 Cspec0 = mix(mat.spec*0.08*mix(vec3(1.0), Ctint, 0.0), Cdlin, mat.metallic);
    vec3 Csheen = vec3(0.0); //Sheen is not used here

    //Diffuse fresnel - 1.0 at normal incidence, 0.5 when grazing
    float FL = SchlickFresnel(NdotL);
    float FV = SchlickFresnel(NdotV);
    float Fd90 = 0.5 + 2.0 * LdotH*LdotH * roughness;
    float Fd = mix(1.0, Fd90, FL) * mix(1.0, Fd90, FV);

    //Based on Hanrahan-Krueger's approximation of isotropic bssrdf
    //1.25 scale is used to (roughly) preserve albedo
    //Fss90 used to "flatten" retroreflections based on roughness
    float Fss90 = LdotH*LdotH*roughness;
    float Fss = mix(1.0, Fss90, FL) * mix(1.0, Fss90, FV);
    float ss = 1.25 * (Fss * (1.0 / (NdotL + NdotV) - 0.5) + 0.5);

    //Specular
    float aspect = sqrt(1-mat.anisotropic*.9);
    float ax = max(.001, roughsqr/aspect);
    float ay = max(.001, roughsqr*aspect);
    float Ds = GTR2_aniso(NdotH, dot(H, X), dot(H, Y), ax, ay);
    float FH = SchlickFresnel(LdotH);
    vec3 Fs = mix(Cspec0, vec3(1.0), FH);
    float Gs;
    Gs  = smithG_GGX_aniso(NdotL, dot(L, X), dot(L, Y), ax, ay);
    Gs *= smithG_GGX_aniso(NdotV, dot(V, X), dot(V, Y), ax, ay);

    //Sheen
    vec3 Fsheen = FH * mat.sheen * Csheen;

    //Clearcoat (ior = 1.5 -> F0 = 0.04)
    float Dr = GTR1(NdotH, mix(0.1, 0.001, mat.clearcoatGloss));
    float Fr = mix(0.04, 1.0, FH);
    float Gr = smithG_GGX(NdotL, 0.25) * smithG_GGX(NdotV, 0.25);

    vec3 result = ((1.0/PI) * mix(Fd, ss, mat.subsurf)*Cdlin + Fsheen)
        * (1.0 - mat.metallic)
        + Gs*Fs*Ds + 0.25*mat.clearcoat*Gr*Fr*Dr;
    return result * max(NdotL, 0.0);
}
