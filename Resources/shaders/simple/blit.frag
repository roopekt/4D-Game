#version 330 core

in vec2 uv;
out vec4 frag_color;

uniform sampler2D texture_to_blit;

void main() {
    frag_color = texture(texture_to_blit, uv);
}