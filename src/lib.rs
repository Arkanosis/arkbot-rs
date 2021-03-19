mod dump;
mod processors;
mod wiki;

use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize)]
struct Config {
    output_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut output_directory = std::env::temp_dir();
        output_directory.push(".arkbot-data");
        Self {
            output_directory: output_directory.to_str().unwrap().to_owned(),
        }
    }
}

pub fn version() -> &'static str {
    return option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
}

pub fn run() {
    let config: Config = confy::load("arkbot").unwrap();
    confy::store("arkbot", &config).unwrap();
    std::fs::create_dir_all(&config.output_directory).expect("Unable to create output directory");

    let mut processors: Vec<Box<dyn processors::Process>> = Vec::new();
    processors.push(Box::new(processors::Commercial::new()));
    //processors.push(Box::new(processors::Debug::new()));
    processors.push(Box::new(processors::Empty::new()));
    //processors.push(Box::new(processors::Impasse::new()));
    processors.push(Box::new(processors::LastEdit::new()));
    processors.push(Box::new(processors::NamespaceRedirect::new()));
    processors.push(Box::new(processors::NoInfobox::new()));
    processors.push(Box::new(processors::NoPortal::new()));

    dump::monitor("frwiki", "pages-articles.xml.bz2", |date, url| {
        println!("Processing dump for {}", &date);

        let mut dump_stream = dump::download(url);
        dump::parse(dump_stream.as_mut(), |page| {
            for processor in processors.iter_mut() {
                processor.process(&page);
            }
        });

        for processor in processors.iter_mut() {
            // TODO publish to wiki instead of writing to file
            processor.write_to_file(&config.output_directory);
        }

        // TODO clear processors (ie. forget about previous dump)
    });
}
