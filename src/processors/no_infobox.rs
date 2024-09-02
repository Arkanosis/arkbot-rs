use crate::processors;
use crate::wiki;

use regex::Regex;

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
        Self {
            titles: Vec::new(),
            infobox: Regex::new(r"(?imsx)\{\{(?:
                abbayes[\s_]homonymes|
                années|
                arrondissements[\s_]homonymes|
                bandeau[\s_]standard[\s_]pour[\s_]page[\s_]d'homonymie|
                batailles[\s_]homonymes|
                cantons[\s_]homonymes|
                chronologie musique|
                chronologie santé et médecine|
                communes[\s_]françaises[\s_]homonymes|
                cycling race/stageinfobox|
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
                palette chronologie croisades|
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
    fn finalize(&mut self) {
        self.titles.sort();
        self.music_titles.sort();
        self.actor_titles.sort();
    }
    fn lines(&self) -> &Vec<String> {
        &self.titles
    }
}
