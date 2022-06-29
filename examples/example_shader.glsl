#stage vertex // Each stage is stored in the same file and seperated using these statements
#version 330 core

layout(location=0) in vec3 aPos;

void main() {
    gl_Position = vec4(aPos, 1.0f);
}

#stage fragment // Each stage is stored in the same file and seperated using these statements
#version 330 core

out vec4 FragColor;

void main() {
    FragColor = vec4(1.0f, 0.0f, 0.0f, 1.0f);
}