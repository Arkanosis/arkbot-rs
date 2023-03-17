mod bot;
mod dump;
mod processors;
mod publishers;
mod wiki;

use publishers::Publish;

use chrono::{
    NaiveDate,
    prelude::Locale,
};

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
    server_url: String,
    script_path: String,
    output_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut output_directory = std::env::temp_dir();
        output_directory.push(".arkbot-data");
        Self {
            login: None,
            password: None,
            server_url: "https://fr.wikipedia.org".to_owned(),
            script_path: "/w".to_owned(),
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

    // TODO Load a list of pairs (processor, List[publishers]) from the configuration
    // TODO Build processors and publishers using factories
    let mut processors: Vec<Box<dyn processors::Process>> = Vec::new();
    //processors.push(Box::new(processors::Commercial::new()));
    //processors.push(Box::new(processors::Debug::new()));
    //processors.push(Box::new(processors::Empty::new()));
    //processors.push(Box::new(processors::Impasse::new()));
    //processors.push(Box::new(processors::LastEdit::new()));
    //processors.push(Box::new(processors::NamespaceRedirect::new()));
    //processors.push(Box::new(processors::NoInfobox::new()));
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

        if let (Some(login), Some(password)) = (&config.login, &config.password) {
            let mut bot = bot::Bot::new(&config.server_url, &config.script_path);

            if bot.login(&login, &password) {

                for processor in processors.iter_mut() {
                    processor.finalize();

                    // TODO FIXME: use a different list of publishers for each processor
                    let publisher = publishers::Wiki::new(&bot,
                        "Projet:Articles sans portail",
                        "Articles sans portail"
                    );

                    publisher.publish(&processor.lines(), &format!("{}", NaiveDate::parse_from_str(&date, "%Y%m%d").unwrap().format_localized("%-d %B %Y", Locale::fr_FR)));
                }

                // TODO clear processors (ie. forget about previous dump)

                state.last_date = date.to_string();

            } else {
                eprintln!("Unable to log in");
            }

        } else {
            eprintln!("Missing login or password in configuration");
        }

    });

    if let Ok(state_file) = File::create(&state_path) {
        let state_writer = BufWriter::new(state_file);
        serde_json::to_writer(state_writer, &state).unwrap();
    }
}

pub fn test() {
    let config = load_config();

    if let (Some(login), Some(password)) = (&config.login, &config.password) {
        let mut bot = bot::Bot::new(&config.server_url, &config.script_path);

        if bot.login(&login, &password) {

            let mut publishers: Vec<Box<dyn publishers::Publish>> = Vec::new();
            publishers.push(Box::new(publishers::Wiki::new(&bot,
                "Utilisateur:Arkbot/Caractères spéciaux à vérifier",
                "Caractères spéciaux à vérifier"
            )));
            //publishers.push(Box::new(publishers::Debug::new()));
            publishers.push(Box::new(publishers::Wiki::new(&bot,
                "Projet:Pages_vides/liste_des_pages_vides",
                "Pages vides"
            )));
            //publishers.push(Box::new(publishers::Wiki::new(&bot,
            //    "Projet:Pages en impasse/liste des pages en impasse",
            //    "Pages en impasse"
            //)));
            publishers.push(Box::new(publishers::Wiki::new(&bot,
                "Projet:Pages les moins modifiées",
                "Pages les moins modifiées"
            )));
            publishers.push(Box::new(publishers::Wiki::new(&bot,
                "Utilisateur:Arkbot/Pages redirigeant hors de l'espace de nom principal",
                "Pages redirigeant hors de l'espace de nom principal"
            )));
            publishers.push(Box::new(publishers::Wiki::new(&bot,
                "Projet:Articles sans infobox",
                "Articles sans infobox"
            )));
            publishers.push(Box::new(publishers::Wiki::new(&bot,
                "Projet:Articles sans portail",
                "Articles sans portail"
            )));

            let publisher = publishers::Wiki::new(&bot, "User:Arktest/test", "Testing arkbot-rs");
            let random_titles:  Vec<String> = (1..500).map(|n| format!("Title {}", n)).collect();
            publisher.publish(&random_titles, &format!("{}", NaiveDate::parse_from_str("20230301", "%Y%m%d").unwrap().format_localized("%-d %B %Y", Locale::fr_FR)));
        } else {
            eprintln!("Unable to log in");
        }
    } else {
        eprintln!("Missing login or password in configuration");
    }
}
