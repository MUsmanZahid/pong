#version 450 core

layout(location = 0) in vec2 vs_Coordinates;
layout(location = 1) in vec4 vs_Colour;
layout(set = 0, binding = 0) uniform sampler2D glyph_coverage_map;

// Outputs
layout(location = 0) out vec4 colour;

void main() {
    colour = vec4(vs_Colour.xyz, texture(glyph_coverage_map, vs_Coordinates));
}
