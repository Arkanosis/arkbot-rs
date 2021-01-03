use crate::processors;
use crate::wiki;

use std::{
    fs::File,
    io::{
        BufWriter,
        Write,
    },
};

pub struct NamespaceRedirect {
    titles: Vec<String>,
}

impl NamespaceRedirect {
    pub fn new() -> Self {
        NamespaceRedirect {
            titles: Vec::new(),
        }
    }
}

fn has_namespace(title: &String) -> bool {
    for namespace in [
        "Média:",
        "Spécial:",
        "Discussion:",
        "Utilisateur:",
        "Discussion utilisateur:",
        "Wikipédia:",
        "Discussion Wikipédia:",
        "Fichier:",
        "Discussion fichier:",
        "MediaWiki:",
        "Discussion MediaWiki:",
        "Modèle:",
        "Discussion modèle:",
        "Aide:",
        "Discussion aide:",
        "Catégorie:",
        "Discussion catégorie:",
        "Portail:",
        "Discussion Portail:",
        "Projet:",
        "Discussion Projet:",
        "Référence:",
        "Discussion Référence:",
        "Module:",
        "Discussion module:",
        "Sujet:",
    ].iter() {
        if title.starts_with(namespace) {
            return true;
        }
    }
    return false;
}

impl processors::Process for NamespaceRedirect {
    fn process(&mut self, page: &wiki::Page) {
        if let Some(target) = &page.target {
            if page.namespace == 0 &&
               !page.title.starts_with("P:") &&
               has_namespace(&target) {
                self.titles.push(page.title.to_string());
            }
        }
    }
    fn write_to_file(&mut self) {
        self.titles.sort();
        const output_file: &str = "data/frwiki-ns_redirects-latest.txt";
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
