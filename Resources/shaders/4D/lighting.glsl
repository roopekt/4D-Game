layout (std140) uniform fragment_uniforms {
    vec4 light_position;
    vec4 light_color;
    vec4 light_ambient_color;
    float light_linear_attenuation;
    float light_quadratic_attenuation;
};

float get_light_attenuation() {
    float dist = distance(light_position, v_in.world_position);
    return 1.0 / (1.0 + light_linear_attenuation * dist + light_quadratic_attenuation * dist*dist); 
}

vec3 get_lit_color(vec3 albedo) {
    vec4 light_direction = normalize(light_position - v_in.world_position);
    float diffuse_strength = max(dot(normalize(v_in.world_normal), light_direction), 0.0);
    vec3 diffuse_light = diffuse_strength * light_color.xyz;

    vec3 light = (light_ambient_color.xyz + diffuse_light) * get_light_attenuation();
    return light * albedo;
}