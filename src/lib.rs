mod bot;
mod dump;
mod processors;
mod wiki;

use directories_next::ProjectDirs;

use serde_derive::{
    Deserialize,
    Serialize,
};

use std::{
    fs::File,
    io::BufReader,
    io::BufWriter,
    path::PathBuf,
};

#[derive(Deserialize, Serialize)]
struct Config {
    login: Option<String>,
    password: Option<String>,
    output_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut output_directory = std::env::temp_dir();
        output_directory.push(".arkbot-data");
        Self {
            login: None,
            password: None,
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

fn get_directories() -> ProjectDirs {
    ProjectDirs::from("net", "Arkanosis", "arkbot").unwrap()
}

fn load_config() -> Config {
    let directories = get_directories();
    let config_directory = directories.config_dir();
    std::fs::create_dir_all(&config_directory).expect("Unable to create configuration directory");
    let mut config_path = config_directory.to_owned();
    config_path.push("config.toml");
    confy::load_path(&config_path).unwrap()
}

fn load_state() -> (State, PathBuf) {
    let directories = get_directories();
    let cache_directory = directories.cache_dir();
    std::fs::create_dir_all(&cache_directory).expect("Unable to create cache directory");
    let mut state_path = cache_directory.to_owned();
    state_path.push("state.json");
    let state = {
        if let Ok(state_file) = File::open(&state_path) {
            let state_reader = BufReader::new(state_file);
            serde_json::from_reader(state_reader).unwrap()
        } else {
            State {
                last_date: "".to_string()
            }
        }
    };
    (state, state_path)
}

pub fn run() {
    let config = load_config();

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

    let (mut state, state_path) = load_state();

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
    let config = load_config();

    if let (Some(login), Some(password)) = (config.login, config.password) {
        let mut bot = bot::Bot::new("http://localhost:8080", "/w");
        if bot.login(&login, &password) {
            if !bot.edit_page("User:Arktest/test", "Testing arkbot-rs", "Hello world!") {
                eprintln!("Unable to edit page");
            }
        } else {
            eprintln!("Unable to log in");
        }
    } else {
        eprintln!("Missing login or password in configuration");
    }
}
