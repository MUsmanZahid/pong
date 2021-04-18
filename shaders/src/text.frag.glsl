#version 450 core

// Inputs
layout(push_constant) uniform constants {
    vec4 colour;
    vec2 coordinate;
} Constants;

layout(set = 0, binding = 0) uniform sampler2D glyph;

// Outputs
layout(location = 0) out vec4 colour;

void main() {
    colour = vec4(Constants.colour.xyz, texture(glyph, Constants.coordinate));
}
