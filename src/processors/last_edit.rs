use crate::processors;
use crate::wiki;

use regex::Regex;

use std::cmp::Ordering;

struct Edit {
    title: String,
    username: String,
    timestamp: String,
}

pub struct LastEdit {
    edits: Vec<Edit>,
    homonymy: Regex,
    titles: Vec<String>,
}

impl LastEdit {
    pub fn new() -> Self {
        Self {
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
            titles: Vec::new(),
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
    fn finalize(&mut self) {
        self.edits.sort_unstable_by(|first_edit, second_edit| {
            match first_edit.timestamp.cmp(&second_edit.timestamp) {
                Ordering::Equal => first_edit.title.cmp(&second_edit.title),
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
            }
        });
	for edit in &self.edits {
	    // TODO FIXME do it properly (lazy?) -- wiki target is "%s — {{a-court|%s}} ({{u\'|%s}})"
	    self.titles.push(format!("{} || FIXME || {} || {} || FIXME", edit.timestamp, edit.title, edit.username));
	}
    }
    fn lines(&self) -> &Vec<String> {
        &self.titles
    }
}
