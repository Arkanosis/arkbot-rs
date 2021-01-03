use crate::processors;
use crate::wiki;

pub struct Empty {
    // TODO
}

impl Empty {
    pub fn new() -> Self {
        Empty {
            // TODO
        }
    }
}

impl processors::Process for Empty {
    fn process(&mut self, page: &wiki::Page) {
        // TODO
    }
    fn write_to_file(&mut self) {
        // TODO
    }
}
