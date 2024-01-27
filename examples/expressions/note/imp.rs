use gtk::{glib, prelude::*, subclass::prelude::*};
use std::cell::OnceCell;

use super::Metadata;

#[derive(Debug, glib::Properties, Default)]
#[properties(wrapper_type = super::Note)]
pub struct Note {
    #[property(get, set, construct_only)]
    pub metadata: OnceCell<Metadata>,
}

#[glib::object_subclass]
impl ObjectSubclass for Note {
    const NAME: &'static str = "Note";
    type Type = super::Note;
}

#[glib::derived_properties]
impl ObjectImpl for Note {}
