#version 450

layout (location=0) in vec2 vPosition;

void main ()
{
	gl_Position = vec4 (vPosition, 0, 1);
}
