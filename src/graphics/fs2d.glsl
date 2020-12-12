//Input from previous stage
in vec3 v_color;
in vec2 v_uv;
in vec3 v_normal;

//Output for current stage
out vec3 frag_color;

void main() {
    // frag_color = vec3(v_uv, 0.0);
    frag_color = v_normal;
}
