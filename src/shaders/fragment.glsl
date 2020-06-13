#version 330

out vec4 FragColor;

in vec3 color;

uniform float clock;

void main() {
   FragColor = vec4(color.x * clock, color.y * clock, color.z * clock, 1.0);
}
