#version 140

in vec3 untransformed_pos;

out vec4 color;

float nice_depth() {
    float ndc = gl_FragCoord.z * 2.0 - 1.0;

    float near = 0.1;
    float far = 500.0;

    return (2.0 * near * far) / (far + near - ndc * (far - near));
}

void main() {
    vec3 v = mod(untransformed_pos, 1.0);
    color = vec4(v, 1.0);
}
