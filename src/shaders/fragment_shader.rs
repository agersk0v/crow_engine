pub const FRAGMENT_SHADER_SOURCE: &str = "#version 330 core

out vec4 FragColor;

in vec2 TexCoord;
in vec3 vertexColor;

uniform sampler2D texture1;
uniform vec3 color;

void main()
{
    vec4 texColor = texture(texture1, TexCoord);
    vec3 finalColor = mix(vertexColor, color, 0.2);
    FragColor = texColor * vec4(finalColor, 1.0);
}\0";
