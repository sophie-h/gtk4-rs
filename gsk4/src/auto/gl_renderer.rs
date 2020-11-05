// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::translate::*;
use gsk_sys;
use std::fmt;
use Renderer;

glib_wrapper! {
    pub struct GLRenderer(Object<gsk_sys::GskGLRenderer, gsk_sys::GskGLRendererClass, GLRendererClass>) @extends Renderer;

    match fn {
        get_type => || gsk_sys::gsk_gl_renderer_get_type(),
    }
}

impl GLRenderer {
    pub fn new() -> GLRenderer {
        assert_initialized_main_thread!();
        unsafe { Renderer::from_glib_full(gsk_sys::gsk_gl_renderer_new()).unsafe_cast() }
    }
}

impl Default for GLRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GLRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GLRenderer")
    }
}
