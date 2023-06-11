#version 330 core

struct AffineTransform4D {
    mat4 matrix;
    vec4 translation;
};

layout (std140) uniform vertex_uniforms {
    AffineTransform4D to_world_transform;
    AffineTransform4D to_view_transform;
    AffineTransform4D to_clip_transform;
    mat4 normal_matrix;
};

in vec4 position;
in vec4 normal;

out GS_IN {
    vec4 clip_position;
    float depth;
    vec4 world_position;
    vec4 world_normal;
} v_out;

vec4 affine_transform(AffineTransform4D transform, vec4 vector) {
    return transform.matrix * vector + transform.translation;
}

void main() {
    v_out.world_position = affine_transform(to_world_transform, position);
    v_out.world_normal = normal_matrix * normal;

    v_out.clip_position = affine_transform(to_clip_transform, position);
    v_out.depth = affine_transform(to_view_transform, position).w;
}