struct AffineTransform4D {
    mat4 matrix;
    vec4 translation;
};

struct OutputData {
    vec4 world_position;
    vec4 world_normal;
    vec4 clip_position;
    float depth;
};

layout (std140) uniform vertex_uniforms {
    AffineTransform4D to_world_transform;
    AffineTransform4D to_view_transform;
    AffineTransform4D to_clip_transform;
    mat4 normal_matrix;
};

in vec4 position;
in vec4 normal;

vec4 affine_transform(AffineTransform4D transform, vec4 vector) {
    return transform.matrix * vector + transform.translation;
}

OutputData get_output_data() {
    vec4 world_position = affine_transform(to_world_transform, position);
    vec4 world_normal = normal_matrix * normal;
    vec4 clip_position = affine_transform(to_clip_transform, position);
    float depth = affine_transform(to_view_transform, position).w;

    return OutputData(world_position, world_normal, clip_position, depth);
}
