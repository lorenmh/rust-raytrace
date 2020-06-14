#version 330

out vec4 FragColor;

in vec3 color;
in vec4 gl_FragCoord;

uniform float clock;

void main() {
   float s = mod(floor(gl_FragCoord.x / 5.0), 2.0) * 0.02;
   float t = mod(floor(gl_FragCoord.y / 5.0), 2.0) * 0.02;

   FragColor = vec4(
      color.x * clock + s + t,
      color.y * clock + s + t,
      color.z * clock + s + t,
      1.0
   );
}
