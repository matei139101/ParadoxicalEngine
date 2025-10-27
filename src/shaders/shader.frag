#version 450

// Input
layout(location = 0) in vec3 frag_color;
layout(location = 1) in vec2 tex_coord;

// Output
layout(location = 0) out vec4 out_color;
layout(binding = 1) uniform sampler2D tex_sampler;

void main() {
    vec4 tex_color = texture(tex_sampler, tex_coord);
    out_color = tex_color;
}
