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
                abbayes[ _]homonymes|
                arrondissements[ _]homonymes|
                bandeau[ _]standard[ _]pour[ _]page[ _]d'homonymie|
                batailles[ _]homonymes|
                cantons[ _]homonymes|
                communes[ _]françaises[ _]homonymes|
                disambig|
                films[ _]homonymes|
                gouvernements[ _]homonymes|
                guerres[ _]homonymes|
                homonyme|
                homonymie|
                hydronymie|
                internationalisation|
                isomérie|
                lieux[ _]homonymes|
                monastères[ _]homonymes|
                paronymie|
                patronyme|
                patronymie|
                personnes[ _]homonymes|
                place[ _]ou[ _]square[ _]homonyme|
                portail|
                prieurés[ _]homonymes|
                prénoms[ _]homonymes|
                rues[ _]homonymes|
                saints[ _]homonymes|
                sigle|
                surnoms[ _]homonymes|
                titres[ _]homonymes|
                toponymie|
                unités[ _]homonymes|
                villes[ _]homonymes|
                voir[ _]homonymes|
                édifices[ _]religieux[ _]homonymes
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
    fn write_to_file(&mut self) {
        self.titles.sort();
        const output_file: &str = "data/frwiki-no_portal-latest.txt";
        if let Ok(file) = File::create(output_file) {
            let mut writer = BufWriter::new(file);
            for title in self.titles.iter() {
                writer.write(title.as_bytes()).unwrap();
                writer.write(b"\n").unwrap();
            }
        } else {
            eprintln!("arkbot: unable to create file: '{}'", output_file);
        }
    }
}
