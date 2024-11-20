// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ColorState, MemoryFormat, Texture};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GdkMemoryTextureBuilder")]
    pub struct MemoryTextureBuilder(Object<ffi::GdkMemoryTextureBuilder, ffi::GdkMemoryTextureBuilderClass>);

    match fn {
        type_ => || ffi::gdk_memory_texture_builder_get_type(),
    }
}

impl MemoryTextureBuilder {
    #[doc(alias = "gdk_memory_texture_builder_new")]
    pub fn new() -> MemoryTextureBuilder {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gdk_memory_texture_builder_new()) }
    }

    #[doc(alias = "gdk_memory_texture_builder_build")]
    pub fn build(&self) -> Texture {
        unsafe { from_glib_full(ffi::gdk_memory_texture_builder_build(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_memory_texture_builder_get_bytes")]
    #[doc(alias = "get_bytes")]
    pub fn bytes(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_none(ffi::gdk_memory_texture_builder_get_bytes(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_get_color_state")]
    #[doc(alias = "get_color_state")]
    #[doc(alias = "color-state")]
    pub fn color_state(&self) -> ColorState {
        unsafe {
            from_glib_none(ffi::gdk_memory_texture_builder_get_color_state(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_get_format")]
    #[doc(alias = "get_format")]
    pub fn format(&self) -> MemoryFormat {
        unsafe {
            from_glib(ffi::gdk_memory_texture_builder_get_format(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_get_height")]
    #[doc(alias = "get_height")]
    pub fn height(&self) -> i32 {
        unsafe { ffi::gdk_memory_texture_builder_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_memory_texture_builder_get_stride")]
    #[doc(alias = "get_stride")]
    pub fn stride(&self) -> usize {
        unsafe { ffi::gdk_memory_texture_builder_get_stride(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_memory_texture_builder_get_update_region")]
    #[doc(alias = "get_update_region")]
    #[doc(alias = "update-region")]
    pub fn update_region(&self) -> Option<cairo::Region> {
        unsafe {
            from_glib_none(ffi::gdk_memory_texture_builder_get_update_region(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_get_update_texture")]
    #[doc(alias = "get_update_texture")]
    #[doc(alias = "update-texture")]
    pub fn update_texture(&self) -> Option<Texture> {
        unsafe {
            from_glib_none(ffi::gdk_memory_texture_builder_get_update_texture(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_get_width")]
    #[doc(alias = "get_width")]
    pub fn width(&self) -> i32 {
        unsafe { ffi::gdk_memory_texture_builder_get_width(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_memory_texture_builder_set_bytes")]
    #[doc(alias = "bytes")]
    pub fn set_bytes(&self, bytes: Option<&glib::Bytes>) {
        unsafe {
            ffi::gdk_memory_texture_builder_set_bytes(
                self.to_glib_none().0,
                bytes.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_set_color_state")]
    #[doc(alias = "color-state")]
    pub fn set_color_state(&self, color_state: Option<&ColorState>) {
        unsafe {
            ffi::gdk_memory_texture_builder_set_color_state(
                self.to_glib_none().0,
                color_state.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_set_format")]
    #[doc(alias = "format")]
    pub fn set_format(&self, format: MemoryFormat) {
        unsafe {
            ffi::gdk_memory_texture_builder_set_format(self.to_glib_none().0, format.into_glib());
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_set_height")]
    #[doc(alias = "height")]
    pub fn set_height(&self, height: i32) {
        unsafe {
            ffi::gdk_memory_texture_builder_set_height(self.to_glib_none().0, height);
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_set_stride")]
    #[doc(alias = "stride")]
    pub fn set_stride(&self, stride: usize) {
        unsafe {
            ffi::gdk_memory_texture_builder_set_stride(self.to_glib_none().0, stride);
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_set_update_region")]
    #[doc(alias = "update-region")]
    pub fn set_update_region(&self, region: Option<&cairo::Region>) {
        unsafe {
            ffi::gdk_memory_texture_builder_set_update_region(
                self.to_glib_none().0,
                mut_override(region.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_set_update_texture")]
    #[doc(alias = "update-texture")]
    pub fn set_update_texture(&self, texture: Option<&impl IsA<Texture>>) {
        unsafe {
            ffi::gdk_memory_texture_builder_set_update_texture(
                self.to_glib_none().0,
                texture.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_memory_texture_builder_set_width")]
    #[doc(alias = "width")]
    pub fn set_width(&self, width: i32) {
        unsafe {
            ffi::gdk_memory_texture_builder_set_width(self.to_glib_none().0, width);
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "bytes")]
    pub fn connect_bytes_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_bytes_trampoline<
            F: Fn(&MemoryTextureBuilder) + Send + Sync + 'static,
        >(
            this: *mut ffi::GdkMemoryTextureBuilder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bytes\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_bytes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "color-state")]
    pub fn connect_color_state_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_color_state_trampoline<
            F: Fn(&MemoryTextureBuilder) + Send + Sync + 'static,
        >(
            this: *mut ffi::GdkMemoryTextureBuilder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::color-state\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_color_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "format")]
    pub fn connect_format_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_format_trampoline<
            F: Fn(&MemoryTextureBuilder) + Send + Sync + 'static,
        >(
            this: *mut ffi::GdkMemoryTextureBuilder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::format\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_format_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "height")]
    pub fn connect_height_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<
            F: Fn(&MemoryTextureBuilder) + Send + Sync + 'static,
        >(
            this: *mut ffi::GdkMemoryTextureBuilder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "stride")]
    pub fn connect_stride_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_stride_trampoline<
            F: Fn(&MemoryTextureBuilder) + Send + Sync + 'static,
        >(
            this: *mut ffi::GdkMemoryTextureBuilder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stride\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_stride_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "update-region")]
    pub fn connect_update_region_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_update_region_trampoline<
            F: Fn(&MemoryTextureBuilder) + Send + Sync + 'static,
        >(
            this: *mut ffi::GdkMemoryTextureBuilder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::update-region\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_update_region_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "update-texture")]
    pub fn connect_update_texture_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_update_texture_trampoline<
            F: Fn(&MemoryTextureBuilder) + Send + Sync + 'static,
        >(
            this: *mut ffi::GdkMemoryTextureBuilder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::update-texture\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_update_texture_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "width")]
    pub fn connect_width_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<
            F: Fn(&MemoryTextureBuilder) + Send + Sync + 'static,
        >(
            this: *mut ffi::GdkMemoryTextureBuilder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v4_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
impl Default for MemoryTextureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for MemoryTextureBuilder {}
unsafe impl Sync for MemoryTextureBuilder {}
