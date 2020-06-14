#version 330

in vec3 attribPosition;
in vec3 attribColor;

uniform mat4 model;
uniform mat4 camera;

out vec3 color;

void main() {
    gl_Position = camera * model * vec4(attribPosition, 1.0);
    color = attribColor;
    //color = vec3(1.0, 0.0, 0.0);
}
