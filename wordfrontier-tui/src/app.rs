use crate::{Config, StatefulList, TabsState};

pub struct App<'a> {
    pub config: Config,
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub db_hub: wordfrontier::DbHub,
    pub word_frontier: StatefulList<wordfrontier::WordFrontierMember>,
    pub translations: StatefulList<wordfrontier::TranslationWithText>,
    pub sentence_memberships: StatefulList<wordfrontier::SentenceMembershipWithTextEtc>,
    pub known_words: StatefulList<wordfrontier::KnownWordWithText>,
}

impl<'a> App<'a> {
    pub fn new(config: Config, db_hub: wordfrontier::DbHub) -> App<'a> {
        let mut app = App {
            config,
            title: " Word Frontier ",
            should_quit: false,
            tabs: TabsState::new(vec!["Sentence Learning", "Tab1", "Tab2"]),
            db_hub,
            word_frontier: StatefulList::new(),
            translations: StatefulList::new(),
            sentence_memberships: StatefulList::new(),
            known_words: StatefulList::new(),
        };
        app.update_word_frontier();
        app.update_translations();
        app.update_sentence_membership();
        app.update_known_words();
        app
    }

    fn update_word_frontier(&mut self) {
        self.word_frontier = StatefulList::with_items(
            self.db_hub
                .query_word_frontier_v(wordfrontier::Range(1, 1), wordfrontier::Order::Descending).expect("uh-oh!")
        );
        // Set the cursor to the 0th element.
        self.word_frontier.next();
        // Reset the cursor for sentence memberships to the 0th element.
        self.sentence_memberships.state.select(None);
        self.sentence_memberships.next();
    }

    fn update_translations(&mut self) {
        self.translations = if let Some(selected_index) = self.word_frontier.state.selected() {
            let sentence_row = &self.word_frontier.items[selected_index];
            StatefulList::with_items(
                self.db_hub
                    .query_translation_with_text_v(sentence_row.sentences_rowid).expect("uh-oh!")
            )
        } else {
            StatefulList::new()
        }
    }

    fn update_sentence_membership(&mut self) {
        let previous_selection = self.sentence_memberships.state.selected();
        self.sentence_memberships = if let Some(selected_index) = self.word_frontier.state.selected() {
            let sentence_row = &self.word_frontier.items[selected_index];
            StatefulList::with_items(
                self.db_hub
                    .query_sentence_membership_with_text_etc_v(sentence_row.sentences_rowid).expect("uh-oh!")
            )
        } else {
            StatefulList::new()
        };
        match previous_selection {
            Some(i) => {
                self.sentence_memberships.state.select(Some(i));
            },
            None => {
                // Set the cursor to the 0th element.
                self.sentence_memberships.next();
            },
        };
    }

    fn update_known_words(&mut self) {
        self.known_words = StatefulList::with_items(
            self.db_hub
                .query_known_word_with_text_v().expect("uh-oh!")
        );
    }

    fn add_selected_sentence_member_to_known_words(&mut self) {
        if let Some(selected_index) = self.sentence_memberships.state.selected() {
            let sentence_membership_with_text_etc = &self.sentence_memberships.items[selected_index];
            self.db_hub.add_known_word(sentence_membership_with_text_etc.word_rowid).expect("uh-oh!");
            self.update_sentence_membership();
            self.update_known_words();
        }
    }
    fn remove_selected_sentence_member_from_known_words(&mut self) {
        if let Some(selected_index) = self.sentence_memberships.state.selected() {
            let sentence_membership_with_text_etc = &self.sentence_memberships.items[selected_index];
            self.db_hub.remove_known_word(sentence_membership_with_text_etc.word_rowid).expect("blahh");
            self.update_sentence_membership();
            self.update_known_words();
        }
    }

    pub fn on_up(&mut self) {
        // TODO: Use the currently focused list
        self.word_frontier.previous();
        self.update_translations();
        self.update_sentence_membership();
    }

    pub fn on_down(&mut self) {
        // TODO: Use the currently focused list
        self.word_frontier.next();
        self.update_translations();
        self.update_sentence_membership();
    }

    pub fn on_right(&mut self) {
        // TODO: Change the currently focused list
        self.sentence_memberships.next();
    }

    pub fn on_left(&mut self) {
        // TODO: Change the currently focused list
        self.sentence_memberships.previous();
    }

    pub fn on_tab(&mut self) {
        self.tabs.next();
    }

    pub fn on_back_tab(&mut self) {
        self.tabs.previous();
    }

    pub fn on_enter(&mut self) {
        self.add_selected_sentence_member_to_known_words();
    }

    pub fn on_backspace(&mut self) {
        self.remove_selected_sentence_member_from_known_words();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_reload(&mut self) {
        // Recompute the word frontier and update state.
        self.update_word_frontier();
        self.update_translations();
        self.update_sentence_membership();
        self.update_known_words();
    }
}
