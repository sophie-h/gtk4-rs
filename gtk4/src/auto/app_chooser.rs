// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::Widget;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct AppChooser(Interface<ffi::GtkAppChooser>) @requires Widget, Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_app_chooser_get_type(),
    }
}

pub const NONE_APP_CHOOSER: Option<&AppChooser> = None;

pub trait AppChooserExt: 'static {
    #[doc(alias = "gtk_app_chooser_get_app_info")]
    #[doc(alias = "get_app_info")]
    fn app_info(&self) -> Option<gio::AppInfo>;

    #[doc(alias = "gtk_app_chooser_get_content_type")]
    #[doc(alias = "get_content_type")]
    fn content_type(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_app_chooser_refresh")]
    fn refresh(&self);
}

impl<O: IsA<AppChooser>> AppChooserExt for O {
    fn app_info(&self) -> Option<gio::AppInfo> {
        unsafe {
            from_glib_full(ffi::gtk_app_chooser_get_app_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn content_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_app_chooser_get_content_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn refresh(&self) {
        unsafe {
            ffi::gtk_app_chooser_refresh(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for AppChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AppChooser")
    }
}
