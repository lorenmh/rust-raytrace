#version 330

in vec2 attribPosition;
in vec3 attribColor;

out vec3 color;

void main() {
    gl_Position = vec4(attribPosition, 0.0, 1.0);
    color = attribColor;
    //color = vec3(1.0, 0.0, 0.0);
}
