use crate::processors;
use crate::wiki;

use regex::Regex;

use std::{
    cmp::Ordering,
    fs::File,
    io::{
        BufWriter,
        Write,
    },
};

struct Edit {
    title: String,
    username: String,
    timestamp: String,
}

pub struct LastEdit {
    edits: Vec<Edit>,
    homonymy: Regex,
}

impl LastEdit {
    pub fn new() -> Self {
        LastEdit {
            edits: Vec::new(),
            homonymy: Regex::new(r"(?imsx)\{\{(?:
                abbayes[\s_]homonymes|
                arrondissements[\s_]homonymes|
                bandeau[\s_]standard[\s_]pour[\s_]page[\s_]d'homonymie|
                batailles[\s_]homonymes|
                cantons[\s_]homonymes|
                communes[\s_]françaises[\s_]homonymes|
                disambig|
                films[\s_]homonymes|
                gouvernements[\s_]homonymes|
                guerres[\s_]homonymes|
                homonyme|
                homonymie|
                hydronymie|
                internationalisation|
                isomérie|
                lieux[\s_]homonymes|
                monastères[\s_]homonymes|
                paronymie|
                patronyme|
                patronymie|
                personnes[\s_]homonymes|
                place[\s_]ou[\s_]square[\s_]homonyme|
                prieurés[\s_]homonymes|
                prénoms[\s_]homonymes|
                rues[\s_]homonymes|
                saints[\s_]homonymes|
                sigle|
                surnoms[\s_]homonymes|
                titres[\s_]homonymes|
                toponymie|
                unités[\s_]homonymes|
                villes[\s_]homonymes|
                édifices[\s_]religieux[\s_]homonymes
            )").unwrap(),
        }
    }
}

impl processors::Process for LastEdit {
    fn process(&mut self, page: &wiki::Page) {
        if page.namespace == 0 {
            if page.target == None {
                if let Some(text) = &page.text {
                    if !self.homonymy.is_match(&text) {
                        if let Some(username) = &page.username {
                            if let Some (timestamp) = &page.timestamp {
                                self.edits.push(Edit {
                                    title: page.title.to_string(),
                                    username: username.to_string(),
                                    timestamp: timestamp[..10].to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    fn write_to_file(&mut self) {
        self.edits.sort_unstable_by(|first_edit, second_edit| {
            match first_edit.timestamp.cmp(&second_edit.timestamp) {
                Ordering::Equal => first_edit.title.cmp(&second_edit.title),
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
            }
        });
        const output_file: &str = "data/frwiki-last_edit-latest.txt";
        if let Ok(file) = File::create(output_file) {
            let mut writer = BufWriter::new(file);
            for edit in self.edits.iter() {
                writer.write(format!("{} || FIXME || {} || {} || FIXME", edit.timestamp, edit.title, edit.username).as_bytes()).unwrap();
                writer.write(b"\n").unwrap();
            }
        } else {
            eprintln!("arkbot: unable to create file: '{}'", output_file);
        }
    }
}
