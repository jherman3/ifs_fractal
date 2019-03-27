#version 330
in vec2 position;
in float hue;
out float v_hue;

uniform mat3 transform;

void main() {
    v_hue = hue;
    vec3 pos = vec3(position, 1.0) * transform;
    gl_Position = vec4(pos.xy, 0.0, 1.0);
}
