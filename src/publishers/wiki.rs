use crate::bot;
use crate::publishers;

pub struct Wiki<'a> {
    bot: &'a bot::Bot,
    root: String,
    summary: String,
}

impl<'a> Wiki<'a> {
    pub fn new(bot: &'a bot::Bot, root: &str, summary: &str) -> Self {
        Wiki {
            bot: bot,
            root: root.to_string(),
            summary: summary.to_string(),
        }
    }
}

impl<'a> publishers::Publish for Wiki<'a> {
    fn publish(&self, titles: &Vec<String>) {
        // TODO FIXME edit wiki
        if !self.bot.edit_page(&self.root, &self.summary, &format!("This is a list of {} titles", titles.len())) {
            eprintln!("Unable to edit page");
        }
    }
}
