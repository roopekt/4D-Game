#version 140

out vec4 frag_color;

float get_nice_depth() {
    float near = 0.1;
    float far = 20;

    float depth = gl_FragCoord.z;
    return (2.0 * near) / (far + near - depth * (far - near));
}

void main() {
    float depth = get_nice_depth();
    frag_color = vec4(depth, depth, depth, 1.0);
}
