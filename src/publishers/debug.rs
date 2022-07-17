use crate::publishers;

pub struct Debug {
    // Nothing
}

impl Debug {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Debug {
            // Nothing
        }
    }
}

impl publishers::Publish for Debug {
    fn publish(&self, titles: &Vec<String>) {
        println!("Number of titles: {}", titles.len());
    }
}
