use crate::processors;
use crate::wiki;

use regex::Regex;

pub struct Impasse {
    titles: Vec<String>,
    link: Regex,
}

impl Impasse {
    pub fn new() -> Self {
        Self {
            titles: Vec::new(),
            link: Regex::new(r"(?imsx)
                # Link
                \[\[.*?\]\]|

                # Exempt from having a link
                \{\{(?:
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
		    infobox|
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
                )
            ").unwrap(),
        }
    }
}

impl processors::Process for Impasse {
    fn process(&mut self, page: &wiki::Page) {
        if page.namespace == 0 {
            if page.target == None {
                if let Some(text) = &page.text {
                    if !self.link.is_match(&text) {
                        self.titles.push(page.title.to_string());
                    }
                }
            }
        }
    }
    fn finalize(&mut self) {
        self.titles.sort();
    }
    fn lines(&self) -> &Vec<String> {
        &self.titles
    }
}
