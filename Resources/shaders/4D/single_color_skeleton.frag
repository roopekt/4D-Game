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
    vec3 lit_color = get_lit_color(albedo);

    vec4 color_at_frustum_border = v_in.clip_position.x > 0.0 ? vec4(1.0, 0.0, 0.0, 0.0) : vec4(0.0, 0.0, 1.0, 0.0);
    float t = abs(v_in.clip_position.x / v_in.depth);
    frag_color = mix(vec4(lit_color, 1.0), color_at_frustum_border, t);
}
