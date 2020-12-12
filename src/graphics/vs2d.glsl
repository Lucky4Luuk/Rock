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

//Output for current stage
out vec3 v_color;
smooth out vec2 v_uv;
smooth out vec3 v_normal;

void main() {
    v_color = color;
    v_uv = uv;

    //Set position for rasterization
    gl_Position = projection * view * offset * vec4(position, 1.);
    v_normal = (normal_matrix * vec4(normalize(normal), 1.)).xyz;
}
