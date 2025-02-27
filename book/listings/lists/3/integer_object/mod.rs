mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;

glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

// ANCHOR: integer_object
impl IntegerObject {
    pub fn new(number: i32) -> Self {
        Object::new(&[("number", &number)]).unwrap()
    }

    pub fn increase_number(self) {
        let old_number = self
            .property("number")
            .expect("The property needs to exist and be readable.")
            .get::<i32>()
            .expect("The property needs to be of type `i32`.");

        self.set_property("number", old_number + 1).unwrap();
    }
}
// ANCHOR_END: integer_object
