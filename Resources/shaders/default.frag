#version 140

uniform vec3 albedo;

uniform vec3 light_position;
uniform vec3 light_color;
uniform vec3 ambient_light_color;
uniform float light_linear_attenuation;
uniform float light_quadratic_attenuation;

in vec3 world_position;
in vec3 world_normal;

out vec4 frag_color;

float get_light_attenuation() {
    float dist = distance(light_position, world_position);
    return 1.0 / (1.0 + light_linear_attenuation * dist + light_quadratic_attenuation * dist*dist); 
}

vec3 get_lit_color(vec3 albedo) {
    vec3 light_direction = normalize(light_position - world_position);
    float diffuse_strength = max(dot(normalize(world_normal), light_direction), 0.0);
    vec3 diffuse_light = diffuse_strength * light_color;

    vec3 light = (ambient_light_color + diffuse_light) * get_light_attenuation();
    return light * albedo;
}

void main() {
    frag_color = vec4(get_lit_color(albedo), 1.0);
}
