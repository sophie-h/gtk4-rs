// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, EventController, Gesture, PropagationLimit, PropagationPhase};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkGestureZoom")]
    pub struct GestureZoom(Object<ffi::GtkGestureZoom, ffi::GtkGestureZoomClass>) @extends Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_zoom_get_type(),
    }
}

impl GestureZoom {
    #[doc(alias = "gtk_gesture_zoom_new")]
    pub fn new() -> GestureZoom {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(ffi::gtk_gesture_zoom_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GestureZoom`] objects.
    ///
    /// This method returns an instance of [`GestureZoomBuilder`](crate::builders::GestureZoomBuilder) which can be used to create [`GestureZoom`] objects.
    pub fn builder() -> GestureZoomBuilder {
        GestureZoomBuilder::new()
    }

    #[doc(alias = "gtk_gesture_zoom_get_scale_delta")]
    #[doc(alias = "get_scale_delta")]
    pub fn scale_delta(&self) -> f64 {
        unsafe { ffi::gtk_gesture_zoom_get_scale_delta(self.to_glib_none().0) }
    }

    #[doc(alias = "scale-changed")]
    pub fn connect_scale_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn scale_changed_trampoline<F: Fn(&GestureZoom, f64) + 'static>(
            this: *mut ffi::GtkGestureZoom,
            scale: std::ffi::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), scale)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scale-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    scale_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureZoom {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GestureZoom`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GestureZoomBuilder {
    builder: glib::object::ObjectBuilder<'static, GestureZoom>,
}

impl GestureZoomBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn n_points(self, n_points: u32) -> Self {
        Self {
            builder: self.builder.property("n-points", n_points),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn propagation_limit(self, propagation_limit: PropagationLimit) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-limit", propagation_limit),
        }
    }

    pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-phase", propagation_phase),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GestureZoom`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GestureZoom {
        self.builder.build()
    }
}
