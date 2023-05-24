#version 140

uniform mat3 mvp_matrix;
uniform vec3 mvp_translation;
uniform mat3 mv_matrix;
uniform vec3 mv_translation;

in vec3 position;

out vec3 untransformed_pos;

void main() {
    untransformed_pos = position;

    vec3 transformed_position = mvp_matrix * position + mvp_translation;
    float depth = (mv_matrix * position + mv_translation).z;
    gl_Position = vec4(transformed_position, depth);
}