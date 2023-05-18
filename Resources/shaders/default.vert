#version 140

uniform mat4 matrix;

in vec3 position;

out vec3 untransformed_pos;

void main() {
    untransformed_pos = position;
    gl_Position = matrix * vec4(position, 1.0);
}