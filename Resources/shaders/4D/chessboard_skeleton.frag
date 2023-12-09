#version 330 core

uniform vec3 albedo_A;
uniform vec3 albedo_B;
uniform float square_width;

in FRAG_IN {
    vec4 world_position;
    vec4 world_normal;
    vec4 clip_position;
    float depth;
} v_in;

out vec4 frag_color;

//@include 4D/lighting.glsl

void main() {
    vec4 p = v_in.world_position / square_width;
    p += vec4(0.5);// this way origin will be at the center of a square, making z-fighting less likely

    bool mod_x = mod(p.x, 2.0) < 1.0;
    bool mod_y = mod(p.y, 2.0) < 1.0;
    bool mod_z = mod(p.z, 2.0) < 1.0;
    bool mod_w = mod(p.w, 2.0) < 1.0;
    bool color_selector = mod_x ^^ mod_y ^^ mod_z ^^ mod_w;

    vec3 albedo = color_selector ? albedo_A : albedo_B;
    vec3 lit_color = get_lit_color(albedo);

    vec4 color_at_frustum_border = v_in.clip_position.x > 0.0 ? vec4(1.0, 0.0, 0.0, 0.0) : vec4(0.0, 0.0, 1.0, 0.0);
    float t = abs(v_in.clip_position.x / v_in.depth);
    frag_color = mix(vec4(lit_color, 1.0), color_at_frustum_border, t);
}
