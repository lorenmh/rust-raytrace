#version 330

out vec4 FragColor;

in vec3 color;
in float distance;
in vec4 gl_FragCoord;

uniform float clock;

void main() {
    float m = 50.0;
    float d;
    if (distance < m) {
        d = distance / m;
    } else {
        d = 1.0;
    }
    float cubic = 1.0 - (d / 1.3) * (d / 1.3);
    FragColor = vec4((color * cubic * 0.6) + (color * 0.4), 1.0);
    //FragColor = vec4(color, 1.0);

   //FragColor = vec4(normalize(color+ light), 1.0);
}
