// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_wayland_sys;
use glib::translate::*;
use glib::GString;
use std::fmt;

glib_wrapper! {
    pub struct WaylandDisplay(Object<gdk_wayland_sys::GdkWaylandDisplay, gdk_wayland_sys::GdkWaylandDisplayClass>) @extends gdk::Display;

    match fn {
        get_type => || gdk_wayland_sys::gdk_wayland_display_get_type(),
    }
}

impl WaylandDisplay {
    pub fn get_startup_notification_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(
                gdk_wayland_sys::gdk_wayland_display_get_startup_notification_id(
                    self.to_glib_none().0,
                ),
            )
        }
    }

    pub fn query_registry(&self, global: &str) -> bool {
        unsafe {
            from_glib(gdk_wayland_sys::gdk_wayland_display_query_registry(
                self.to_glib_none().0,
                global.to_glib_none().0,
            ))
        }
    }

    pub fn set_cursor_theme(&self, theme: &str, size: i32) {
        unsafe {
            gdk_wayland_sys::gdk_wayland_display_set_cursor_theme(
                self.to_glib_none().0,
                theme.to_glib_none().0,
                size,
            );
        }
    }

    pub fn set_startup_notification_id(&self, startup_id: &str) {
        unsafe {
            gdk_wayland_sys::gdk_wayland_display_set_startup_notification_id(
                self.to_glib_none().0,
                startup_id.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for WaylandDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WaylandDisplay")
    }
}
