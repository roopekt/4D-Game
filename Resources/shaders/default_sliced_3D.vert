#version 330 core

struct AffineTransform3D {
    mat3 matrix;
    vec3 translation;
};

layout (std140) uniform vertex_uniforms {
    AffineTransform3D to_world_transform;
    AffineTransform3D to_view_transform;
    AffineTransform3D to_clip_transform;
    mat3 normal_matrix;
};

in vec3 position;
in vec3 normal;

out GS_IN {
    vec3 world_position;
    vec3 world_normal;
} v_out;

vec3 affine_transform(AffineTransform3D transform, vec3 vector) {
    return transform.matrix * vector + transform.translation;
}

void main() {
    v_out.world_position = affine_transform(to_world_transform, position);
    v_out.world_normal = normal_matrix * normal;

    vec3 clip_position = affine_transform(to_clip_transform, position);
    float depth = affine_transform(to_view_transform, position).z;
    gl_Position = vec4(clip_position, depth);
}