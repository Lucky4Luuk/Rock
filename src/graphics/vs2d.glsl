uniform mat4 offset;

//MVP
uniform mat4 projection;
uniform mat4 view;
uniform mat4 normal_matrix;

//Vertex semantics
in vec3 position;
in vec3 color;
in vec2 uv;
in vec3 normal;
in vec4 tangent;

//Output for current stage
out vec3 v_color;
out vec2 v_uv;
out vec3 v_normal;
out vec3 v_wpos;
out vec4 v_tangent;

void main() {
    v_color = color;
    v_uv = uv;

    //Set position for rasterization
    gl_Position = projection * view * offset * vec4(position, 1.);
    v_wpos = (offset * vec4(position, 1.)).xyz;
    v_normal = normalize(mat3(normal_matrix) * normalize(normal));
    v_tangent = vec4(mat3(normal_matrix) * tangent.xyz, tangent.w);
}
