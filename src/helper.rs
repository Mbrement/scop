use gl::types::GLuint;

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
  unsafe { gl::ClearColor(r, g, b, a) }
}
pub fn clear(mask: u32) {
  unsafe { gl::Clear(mask) }
}

/// Basic wrapper for a [Vertex Array
/// Object](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object).
pub struct VertexArray(pub GLuint);
impl VertexArray {
///Creates a new vertex array object
  pub fn new() -> Option<Self> {
    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };
    if vao != 0 {
      Some(Self(vao))
    } else {
      None
    }
  }

  /// Bind this vertex array as the current vertex array object
  pub fn bind(&self) {
    unsafe { gl::BindVertexArray(self.0) }
  }

  /// Clear the current vertex array object binding.
  pub fn clear_binding() {
    unsafe { gl::BindVertexArray(0) }
  }
}