#version 330 core
layout (location = 0) in vec4 pos;
layout (location = 1) in vec2 txt;
layout (location = 2) in vec4 col;

uniform mat4 u_mvp;

out vec2 v_text;
out vec4 v_col;

void main()
{
    gl_Position = u_mvp * pos;
    v_text = txt;
    v_col = col;
}

# frag
#version 330 core
layout (location = 0) out vec4 color;

in vec2 v_text;
in vec4 v_col;

uniform sampler2D u_tex;

void main()
{
    color = v_col;
    // color = vec4(v_col.xyz, 1.0 * v_col.w);
    // color = vec4(textColor, 1.0) * sampled;
}
