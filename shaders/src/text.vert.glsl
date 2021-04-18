#version 450 core

// Inputs
layout(location = 0) in vec2 vi_Position;
layout(location = 1) in vec2 vi_Coordinates;
layout(location = 2) in vec4 vi_Colour;

// Outputs
layout(location = 0) out vec2 vs_Coordinates;
layout(location = 1) out vec4 vs_Colour;

void main() {
    gl_Position = vec4(vi_Position, 0.0, 1.0);
    vs_Coordinates = vi_Coordinates;
    vs_Colour = vi_Colour;
}
