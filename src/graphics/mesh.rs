use std::rc::Rc;

use luminance_sdl2::GL33Surface;
use luminance_gl::GL33;

use luminance::context::GraphicsContext as _;
use luminance::tess::{Tess, TessBuilder};

use super::VertexType;

#[derive(Clone)]
pub struct Mesh {
    tess: Rc<Tess<GL33, VertexType>>
}

impl Mesh {
    pub fn new<F>(surface: &mut luminance_sdl2::GL33Surface, func: F) -> Self
    where
        F: Fn(TessBuilder<GL33, ()>) -> TessBuilder<GL33, VertexType>
    {
        let builder = func(TessBuilder::<GL33, ()>::new(surface));
        let mesh = builder.build().expect("Failed to create mesh!"); //TODO: Error handling

        Self {
            tess: Rc::new(mesh)
        }
    }

    pub fn tess(&self) -> &Tess<GL33, VertexType> {
        &self.tess
    }
}
