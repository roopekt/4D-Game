#version 330 core

uniform vec3 albedo;

in FRAG_IN {
    vec4 world_position;
    vec4 world_normal;
    vec4 clip_position;
    float depth;
} v_in;

out vec4 frag_color;

//@include 4D/lighting.glsl

void main() {
    frag_color = vec4(get_lit_color(albedo), 1.0);
}
