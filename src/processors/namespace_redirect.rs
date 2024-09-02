use crate::processors;
use crate::wiki;

pub struct NamespaceRedirect {
    titles: Vec<String>,
}

impl NamespaceRedirect {
    pub fn new() -> Self {
        Self {
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
    fn finalize(&mut self) {
        self.titles.sort();
    }
    fn lines(&self) -> &Vec<String> {
        &self.titles
    }
}
