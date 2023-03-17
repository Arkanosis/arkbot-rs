use crate::processors;
use crate::wiki;

use regex::Regex;

pub struct NoPortal {
    titles: Vec<String>,
    portal: Regex,
}

impl NoPortal {
    pub fn new() -> Self {
        NoPortal {
            titles: Vec::new(),
            portal: Regex::new(r"(?imsx)\{\{(?:
                abbatiales[\s_]homonymes|
                abbayes[\s_]homonymes|
                arrondissements[\s_]homonymes|
                bandeau[\s_]standard[\s_]pour[\s_]page[\s_]d'homonymie|
                basiliques[\s_]homonymes|
                batailles[\s_]homonymes|
                calvaires[\s_]homonymes|
                cantons[\s_]homonymes|
                cathédrales[\s_]homonymes|
                chapelles[\s_]homonymes|
                chartreuses[\s_]homonymes|
                collégiales[\s_]homonymes|
                communes[\s_]françaises[\s_]homonymes|
                couvents[\s_]homonymes|
                disambig|
                édifices[\s_]religieux[\s_]homonymes|
                églises[\s_]homonymes|
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
                prénoms[\s_]homonymes|
                prieurés[\s_]homonymes|
                pseudonymes[\s_]homonymes|
                rues[\s_]homonymes|
                saints[\s_]homonymes|
                sigle|
                surnoms[\s_]homonymes|
                titres[\s_]homonymes|
                toponymie|
                unités[\s_]homonymes|
                villes[\s_]homonymes
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
    fn finalize(&mut self) {
        self.titles.sort();
    }
    fn lines(&self) -> &Vec<String> {
        &self.titles
    }
}
