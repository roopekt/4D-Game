#version 330 core

//@include 3D/vertex_general.glsl

out FRAG_IN {
    vec3 world_position;
    vec3 world_normal;
    vec3 clip_position;
    float depth;
} v_out;

void main() {
    OutputData output_data = get_output_data();

    v_out.world_position = output_data.world_position;
    v_out.world_normal = output_data.world_normal;
    v_out.clip_position = output_data.clip_position;
    v_out.depth = output_data.depth;
    gl_Position = vec4(output_data.clip_position, output_data.depth);
}