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
    fn publish(&self, titles: &Vec<String>, dump: &str) {
        println!("Number of titles: {}, dump: {}", titles.len(), dump);
    }
}
