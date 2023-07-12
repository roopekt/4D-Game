struct AffineTransform3D {
    mat3 matrix;
    vec3 translation;
};

struct OutputData {
    vec3 world_position;
    vec3 world_normal;
    vec3 clip_position;
    float depth;
};

layout (std140) uniform vertex_uniforms {
    AffineTransform3D to_world_transform;
    AffineTransform3D to_view_transform;
    AffineTransform3D to_clip_transform;
    mat3 normal_matrix;
};

in vec3 position;
in vec3 normal;

vec3 affine_transform(AffineTransform3D transform, vec3 vector) {
    return transform.matrix * vector + transform.translation;
}

OutputData get_output_data() {
    vec3 world_position = affine_transform(to_world_transform, position);
    vec3 world_normal = normal_matrix * normal;
    vec3 clip_position = affine_transform(to_clip_transform, position);
    float depth = affine_transform(to_view_transform, position).z;

    return OutputData(world_position, world_normal, clip_position, depth);
}
