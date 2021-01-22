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

pub struct NoInfobox {
    titles: Vec<String>,
    infobox: Regex,

    music_titles: Vec<String>,
    music: Regex,

    actor_titles: Vec<String>,
    actor: Regex,
}

impl NoInfobox {
    pub fn new() -> Self {
        NoInfobox {
            titles: Vec::new(),
            infobox: Regex::new(r"(?imsx)\{\{(?:
                abbayes[\s_]homonymes|
                années\}\}|
                arrondissements[\s_]homonymes|
                bandeau[\s_]standard[\s_]pour[\s_]page[\s_]d'homonymie|
                batailles[\s_]homonymes|
                cantons[\s_]homonymes|
                chronologie musique\}\}|
                chronologie santé et médecine\}\}|
                communes[\s_]françaises[\s_]homonymes|
                cycling race/stageinfobox\}\}|
                disambig|
                films[\s_]homonymes|
                gouvernements[\s_]homonymes|
                guerres[\s_]homonymes|
                homonyme|
                homonymie|
                hydronymie|
                infobox|
                internationalisation|
                isomérie|
                lieux[\s_]homonymes|
                monastères[\s_]homonymes|
                palette chronologie croisades\}\}|
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

            music_titles: Vec::new(),
            music: Regex::new(r"(?imsx)(?:
                cat[ée]gor(y|ie)\s*?:\s*?album[\s_]musical
            )").unwrap(),

            actor_titles: Vec::new(),
            actor: Regex::new(r"(?imsx)(?:
                cat[ée]gor(y|ie)\s*?:\s*?act(eur|rice)
            )").unwrap(),
        }
    }
}

fn write_titles(titles: &mut Vec<String>, output_file: &str) {
    titles.sort();
    if let Ok(file) = File::create(output_file) {
        let mut writer = BufWriter::new(file);
        for title in titles.iter() {
            writer.write(title.as_bytes()).unwrap();
            writer.write(b"\n").unwrap();
        }
    } else {
        eprintln!("arkbot: unable to create file: '{}'", output_file);
    }
}

impl processors::Process for NoInfobox {
    fn process(&mut self, page: &wiki::Page) {
        if page.namespace == 0 {
            if page.target == None {
                if let Some(text) = &page.text {
                    if !self.infobox.is_match(&text) {
                        self.titles.push(page.title.to_string());
                        if self.music.is_match(&text) {
                            self.music_titles.push(page.title.to_string());
                        }
                        if self.actor.is_match(&text) {
                            self.actor_titles.push(page.title.to_string());
                        }
                    }
                }
            }
        }
    }
    fn write_to_file(&mut self) {
	write_titles(&mut self.titles, "data/frwiki-no_infobox-latest.txt");
	write_titles(&mut self.music_titles, "data/frwiki-no_infobox_music-latest.txt");
	write_titles(&mut self.actor_titles, "data/frwiki-no_infobox_actor-latest.txt");
    }
}
