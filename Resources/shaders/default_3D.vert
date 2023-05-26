#version 140

uniform mat3 to_world_matrix;
uniform vec3 to_world_translation;
uniform mat3 to_view_matrix;
uniform vec3 to_view_translation;
uniform mat3 to_clip_matrix;
uniform vec3 to_clip_translation;
uniform mat3 normal_matrix;

in vec3 position;
in vec3 normal;

out vec3 world_position;
out vec3 world_normal;

void main() {
    world_position = to_world_matrix * position + to_world_translation;
    world_normal = normal_matrix * normal;

    vec3 clip_position = to_clip_matrix * position + to_clip_translation;
    float depth = (to_view_matrix * position + to_view_translation).z;
    gl_Position = vec4(clip_position, depth);
}