// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FileChooser;
use glib::translate::*;
use glib::IsA;

pub trait FileChooserExtManual: 'static {
    #[doc(alias = "gtk_file_chooser_add_choice")]
    fn add_choice(&self, id: &str, label: &str, options: &[(Option<&str>, Option<&str>)]);
}

impl<O: IsA<FileChooser>> FileChooserExtManual for O {
    fn add_choice(&self, id: &str, label: &str, options: &[(Option<&str>, Option<&str>)]) {
        unsafe {
            let options_ids: Vec<_> = options.into_iter().map(|o| o.0.to_glib_none().0).collect();
            let options_labels: Vec<_> =
                options.into_iter().map(|o| o.1.to_glib_none().0).collect();

            ffi::gtk_file_chooser_add_choice(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                label.to_glib_none().0,
                mut_override(options_ids.as_ptr()),
                mut_override(options_labels.as_ptr()),
            );
        }
    }
}
