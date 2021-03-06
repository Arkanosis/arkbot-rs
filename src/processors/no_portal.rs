use crate::processors;
use crate::wiki;

use regex::Regex;

use std::{
    fs::File,
    io::{
        BufWriter,
        Write,
    },
};

pub struct NoPortal {
    titles: Vec<String>,
    portal: Regex,
}

impl NoPortal {
    pub fn new() -> Self {
        NoPortal {
            titles: Vec::new(),
            portal: Regex::new(r"(?imsx)\{\{(?:
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
                portail|
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

impl processors::Process for NoPortal {
    fn process(&mut self, page: &wiki::Page) {
        if page.namespace == 0 {
            if page.target == None {
                if let Some(text) = &page.text {
                    if !self.portal.is_match(&text) {
                        self.titles.push(page.title.to_string());
                    }
                }
            }
        }
    }
    fn write_to_file(&mut self, output_directory: &str) {
        self.titles.sort();
        let output_file = format!("{}/frwiki-no_portal-latest.txt", output_directory);
        if let Ok(file) = File::create(&output_file) {
            let mut writer = BufWriter::new(file);
            for title in self.titles.iter() {
                writer.write(title.as_bytes()).unwrap();
                writer.write(b"\n").unwrap();
            }
        } else {
            eprintln!("arkbot: unable to create file: '{}'", &output_file);
        }
    }
}
