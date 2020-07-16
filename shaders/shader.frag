#version 450

layout(location=0) in vec3 v_color;
layout(location=0) out vec4 f_color;

layout(set=0, binding=0)
uniform Uniforms {
     vec3 u_color;
};

void main() {
    f_color = vec4(u_color , 1.0);
}
