#version 330

in vec3 attribPosition;
in vec3 attribColor;

uniform mat4 model;
uniform mat4 camera;

out vec3 color;
out float distance;

void main() {
    vec4 p = model * vec4(attribPosition, 1.0);
    gl_Position = camera * p;

    color = attribColor;

    distance = length(p);
}
