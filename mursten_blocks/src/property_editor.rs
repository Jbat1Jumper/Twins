use mursten::{Updater, Data};
use properties::{Property, GetProperties};

pub struct PropertyEditor {
}

impl<B, D> Updater<B, D> for PropertyEditor
where D: Data + GetProperties {
    fn update(&mut self, _: &mut B, data: &mut D) {
    }
}
