mod bot;
mod dump;
mod processors;
mod wiki;

use serde_derive::{
    Deserialize,
    Serialize,
};

use std::{
    fs::File,
    io::BufReader,
    io::BufWriter,
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

#[derive(Serialize, Deserialize)]
struct State {
    last_date: String,
}

pub fn version() -> &'static str {
    return option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
}

pub fn run() {
    let directories = directories_next::ProjectDirs::from("net", "Arkanosis", "arkbot").unwrap();

    let config_directory = directories.config_dir();
    std::fs::create_dir_all(&config_directory).expect("Unable to create configuration directory");
    let mut config_path = config_directory.to_owned();
    config_path.push("config.toml");
    let config: Config = confy::load_path(&config_path).unwrap();

    let cache_directory = directories.cache_dir();
    std::fs::create_dir_all(&cache_directory).expect("Unable to create cache directory");
    let mut state_path = cache_directory.to_owned();
    state_path.push("state.json");
    let mut state = {
        if let Ok(state_file) = File::open(&state_path) {
            let state_reader = BufReader::new(state_file);
            serde_json::from_reader(state_reader).unwrap()
        } else {
            State {
                last_date: "".to_string()
            }
        }
    };

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
        if date == state.last_date {
            return;
        }

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

        state.last_date = date.to_string();
    });

    if let Ok(state_file) = File::create(&state_path) {
        let state_writer = BufWriter::new(state_file);
        serde_json::to_writer(state_writer, &state).unwrap();
    }
}

pub fn test() {
    let mut bot = bot::Bot::new("http://localhost:8080", "/w");
    if bot.login("LOGIN", "PASSWORD") {
        if !bot.edit_page("User:Arktest/test", "Testing arkbot-rs", "Hello world!") {
            eprintln!("Unable to edit page");
        }
    } else {
        eprintln!("Unable to log in");
    }
}
