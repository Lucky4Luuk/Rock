uniform mat4 offset;

//MVP
uniform mat4 projection;
uniform mat4 view;

in vec3 position;
in vec3 color;

out vec3 v_color;

void main() {
    v_color = color;
    gl_Position = projection * view * offset * vec4(position, 1.);
}
