#version 330 core
layout (points) in;
layout (line_strip, max_vertices = 2) out;

in GS_IN {
    vec3 clip_position;
    float depth;
    vec3 world_position;
    vec3 world_normal;
} v_in[];

out FRAG_IN {
    vec3 world_position;
    vec3 world_normal;
    vec3 clip_position;
    float depth;
} v_out;

void main() {
    //note that perspective division happens after the geometry shader has run
    gl_Position = vec4(
        -v_in[0].depth,
        v_in[0].clip_position.y,
        v_in[0].clip_position.z,
        v_in[0].depth
    );
    v_out.world_position = v_in[0].world_position;
    v_out.world_normal = v_in[0].world_normal;
    v_out.clip_position = v_in[0].clip_position;
    v_out.depth = v_in[0].depth;
    EmitVertex();

    gl_Position = vec4(
        v_in[0].depth,
        v_in[0].clip_position.y,
        v_in[0].clip_position.z,
        v_in[0].depth
    );
    v_out.world_position = v_in[0].world_position;
    v_out.world_normal = v_in[0].world_normal;
    v_out.clip_position = v_in[0].clip_position;
    v_out.depth = v_in[0].depth;
    EmitVertex();

    EndPrimitive();
}