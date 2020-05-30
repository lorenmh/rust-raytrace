#version 150

in vec4 gl_FragCoord;
out vec4 out_color;

void main() {
   out_color = vec4(gl_FragCoord.x, gl_FragCoord.y, 0.0, 1.0);
}
