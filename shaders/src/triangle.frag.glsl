#version 450 core

layout(location = 0) in vec2 vposition;
layout(binding = 0) uniform sampler2D image;

layout(location = 0) out vec4 color;

void main() {
    color = texture(image, vposition);
}
