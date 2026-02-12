#version 450

//Push constants
layout(push_constant) uniform PushConstants {
    mat4 mvp;
} pc;

// Input
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;
layout(location = 2) in vec2 tex_coord;

// Output
layout(location = 0) out vec3 frag_color;
layout(location = 1) out vec2 frag_tex_coord;

void main() {
    gl_Position = pc.mvp * vec4(position, 1.0);
    frag_color = color;
    frag_tex_coord = tex_coord;
}
