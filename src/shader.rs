pub const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

pub const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;
  void main() {
  float x = gl_FragCoord.x / 800.0;
  float y = gl_FragCoord.y / 600.0;
  float z = gl_FragCoord.z;
	final_color = vec4(x, y, z, 1.0);
  }
"#;
