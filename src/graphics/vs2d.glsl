uniform mat4 offset;

in vec3 position;
in vec3 color;

out vec3 v_color;

void main() {
    v_color = color;
    gl_Position = offset * vec4(position, 1.);
}
