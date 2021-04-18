#version 450 core

layout(location = 0) in vec2 vi_Position;

void main() {
    gl_Position = vec4(vi_Position, 0.0, 1.0);
}
