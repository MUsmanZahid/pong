#version 450 core

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 vi_tc;

layout(location = 0) out vec2 vposition;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vposition = vi_tc;
}
