use crate::bot;
use crate::publishers;

use std::{
    cmp::min,
    fmt::Write
};

pub struct Wiki<'a> {
    bot: &'a bot::Bot,
    root: String,
    summary: String,
}

impl<'a> Wiki<'a> {
    pub fn new(bot: &'a bot::Bot, root: &str, summary: &str) -> Self {
        Self {
            bot: bot,
            root: root.to_string(),
            summary: summary.to_string(),
        }
    }
}

const LINES_PER_COLUMN: usize = 33;
const COLUMN_PER_SECTION: usize = 3;
const SECTIONS_PER_PAGE: usize = 5;
const LINES_PER_SECTION: usize = LINES_PER_COLUMN * COLUMN_PER_SECTION;
const LINES_PER_PAGE: usize = LINES_PER_SECTION * SECTIONS_PER_PAGE;

impl<'a> publishers::Publish for Wiki<'a> {
    fn publish(&self, titles: &Vec<String>, dump: &str) {
        let page_count = (titles.len() + LINES_PER_PAGE - 1) / LINES_PER_PAGE;
        for page_id in 1..=page_count {
            let page_start = (page_id - 1) * LINES_PER_PAGE + 1;
            let page_end = min(page_id * LINES_PER_PAGE, titles.len());
            let mut text = String::new();
            write!(&mut text, r#"{{{{Mise à jour bot|Arkanosis}}}}

== {} ({} à {}) ==

{{{{../intro}}}}

Dernière mise à jour le ~~~~~ avec le dump du {}.
"#, &self.summary, page_start, page_end, dump).unwrap();
            for section_id in 1..=SECTIONS_PER_PAGE {
                let section_start = page_start + (section_id - 1) * LINES_PER_SECTION;
                let section_end = min(page_start + section_id * LINES_PER_SECTION - 1, titles.len());
                if section_start > section_end {
                    break;
                }
                write!(&mut text, r#"
=== {} à {} ===

<ol start="{}" style="-moz-column-count:{}; column-count:{};">
"#, section_start, section_end, section_start, COLUMN_PER_SECTION, COLUMN_PER_SECTION).unwrap();
                for column_id in 1..=COLUMN_PER_SECTION {
                    let column_start = section_start + (column_id - 1) * LINES_PER_COLUMN;
                    let column_end = min(section_start + column_id * LINES_PER_COLUMN - 1, titles.len());
                    if column_start > column_end {
                        break;
                    }
                    write!(&mut text, r#"
<!-- {} à {} -->

"#, column_start, column_end).unwrap();
                    for line_id in column_start..=min(column_end, titles.len()) {
                        writeln!(&mut text, r#"<li>[[{}]]</li>"#, titles.get(line_id - 1).unwrap()).unwrap();
                    }
                }
                write!(&mut text, r#"
</ol>
"#).unwrap();
            }
            if !self.bot.edit_page(&format!("{}/{}", self.root, page_id), &format!("bot: {} au {}, {} à {}", &self.summary, dump, page_start, page_end), &text) {
                eprintln!("Unable to edit page {}/{}", self.root, page_id);
            }
        }
    }
}
