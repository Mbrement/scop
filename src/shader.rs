pub const VERT_SHADER_UNIFORM: &str = r#"#version 460 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

// pub const VERT_SHADER: &str = r#"#version 460 core
//   layout (location = 0) in vec3 pos;
//   layout (location = 1) in vec3 color;

//   out vec4 vertex_color;

//   void main() {
//     gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
//     vertex_color = vec4(color, 1.0);
//   }
// "#;

// rotation variant
pub const VERT_SHADER: &str = r#"#version 460 core
  uniform mat4 model;
  uniform mat4 view;
  uniform mat4 projection;

  layout (location = 0) in vec3 pos;
  layout (location = 1) in vec2 tex;

  out vec2 frag_tex;

  void main() {
    gl_Position = projection * view * model * vec4(pos, 1.0);
    frag_tex = tex;
  }
"#;

// texture variante
// pub const VERT_SHADER: &str = r#"#version 460 core
//   uniform mat4 transform;
  
//   layout (location = 0) in vec3 pos;
//   layout (location = 1) in vec2 tex;

//   out vec2 frag_tex;

//   void main() {
//     gl_Position = transform * vec4(pos, 1.0);
//     frag_tex = tex;
//   }
// "#;

// OLD
// pub const FRAG_SHADER: &str = r#"#version 460 core
//   out vec4 final_color;
//   void main() {
//   float x = gl_FragCoord.x / 800.0;
//   float y = gl_FragCoord.y / 600.0;
//   float z = gl_FragCoord.z;
// 	final_color = vec4(x, y, z, 1.0);
//   }
// "#;

//

// uniform variante
pub const FRAG_SHADER_UNIFORM: &str = r#"#version 460 core
  uniform vec4 uni_color;

  out vec4 final_color;

  void main() {
    final_color = uni_color;
  }
"#;

// texture variante and position
// pub const FRAG_SHADER: &str = r#"#version 460 core
//   uniform sampler2D logo_texture;
//   uniform sampler2D garris_texture;
//   uniform float time;

//   in vec2 frag_tex;

//   out vec4 final_color;

//   void main() {
//     final_color = mix(
// 	texture(logo_texture, frag_tex),
// 	texture(garris_texture, frag_tex),
// 	time);
// 		  }
// "#;

// tex variante
pub const FRAG_SHADER: &str = r#"#version 460 core
  uniform sampler2D logo_texture;
  uniform sampler2D garris_texture;
  uniform float time;

  in vec4 frag_color;
  in vec2 frag_tex;

  out vec4 final_color;

  void main() {
    final_color = mix(
	texture(logo_texture, frag_tex), 
	texture(garris_texture, frag_tex), 
	time);
		  }
"#;
