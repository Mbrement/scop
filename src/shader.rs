pub const VERT_SHADER_UNIFORM: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

pub const VERT_SHADER: &str = r#"#version 460 core
  layout (location = 0) in vec3 pos;

  out vec4 vertex_color;

  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    vertex_color = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

// OLD
// pub const FRAG_SHADER: &str = r#"#version 330 core
//   out vec4 final_color;
//   void main() {
//   float x = gl_FragCoord.x / 800.0;
//   float y = gl_FragCoord.y / 600.0;
//   float z = gl_FragCoord.z;
// 	final_color = vec4(x, y, z, 1.0);
//   }
// "#;

pub const FRAG_SHADER: &str = r#"#version 460 core

  out vec4 final_color;
    in vec4 vertex_color;
  void main() {
	float x = ((gl_FragCoord.x + vertex_color.x)  / 800.0 );
	float y = ((gl_FragCoord.y + vertex_color.y)  / 600.0 );
	float z = (gl_FragCoord.z + vertex_color.z) / 2;
	final_color = vec4(x, y, z, 1.0);
	}
	"#;

// uniform variante
pub const FRAG_SHADER_UNIFORM: &str = r#"#version 330 core
  uniform vec4 uni_color;

  out vec4 final_color;

  void main() {
    final_color = uni_color;
  }
"#;
