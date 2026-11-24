#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 VertexColor;
out vec4 aColor;

uniform mat3 uTransform;

void main()
{
    vec3 transformedPos = uTransform * vec3(aPos.xy, 1.0);
    gl_Position = vec4(transformedPos.xy, aPos.z, 1.0);

    aColor = VertexColor;
}