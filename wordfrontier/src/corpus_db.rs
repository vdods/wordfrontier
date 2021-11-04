use crate::Result;
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    io::BufRead,
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
pub enum Order {
    Ascending,
    Descending,
    Unordered,
}

// TODO: Use appropriate type with trait with comparison operators
pub struct Range(pub i32, pub i32);

pub struct Lang {
    pub short: String,
    pub long: String,
}

pub struct LangRow {
    pub langs_rowid: i32,
    pub short: String,
    pub long: String,
}

// pub struct Sentence {
//     pub lang_rowid: i32,
//     pub text: String,
// }

#[derive(Debug)]
pub struct SentenceRow {
    pub sentences_rowid: i32,
    pub lang_rowid: i32,
    pub text: String,
}

#[derive(Debug)]
pub struct WordFrontierMember {
    pub sentences_rowid: i32,
    pub lang_rowid: i32,
    pub text: String,
    pub unknown_word_count: i32,
    pub unknown_word_freq: i32,
}

pub struct WordRow {
    pub words_rowid: i32,
    pub lang_rowid: i32,
    pub text: String,
    pub freq: i32,
}

pub struct SentenceMembershipRow {
    pub sentence_memberships_rowid: i32,
    pub sentence_rowid: i32,
    pub word_rowid: i32,
}

pub struct SentenceMembershipWithTextEtc {
    pub sentence_memberships_rowid: i32,
    pub sentence_rowid: i32,
    pub word_rowid: i32,
    pub word_text: String,
    pub word_freq: i32,
    pub word_is_known: bool,
}

pub struct TranslationRow {
    pub translations_rowid: i32,
    pub target_lang_sentence_rowid: i32,
    pub reference_lang_sentence_rowid: i32,
}

pub struct TranslationWithText {
    pub translations_rowid: i32,
    pub target_lang_sentence_rowid: i32,
    pub reference_lang_sentence_rowid: i32,
    pub reference_lang_sentence_text: String,
}

#[derive(Debug)]
pub struct WordFrontierWithTranslation {
    pub target_lang_sentence_rowid: i32,
    pub target_lang_text: String,
    pub reference_lang_sentence_rowid: i32,
    pub reference_lang_text: String,
}

pub struct KnownWordRow {
    pub known_words_rowid: i32,
    pub word_rowid: i32,
}

pub struct KnownWordWithText {
    pub known_words_rowid: i32,
    pub word_rowid: i32,
    pub word_text: String,
}

// TODO: Figure out how to make a derive macro for this.
impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for LangRow {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(LangRow {
            langs_rowid: row.get(0)?,
            short: row.get(1)?,
            long: row.get(2)?,
        })
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for SentenceRow {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(SentenceRow {
            sentences_rowid: row.get(0)?,
            lang_rowid: row.get(1)?,
            text: row.get(2)?,
        })
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for WordFrontierMember {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(WordFrontierMember {
            sentences_rowid: row.get(0)?,
            lang_rowid: row.get(1)?,
            text: row.get(2)?,
            unknown_word_count: row.get(3)?,
            unknown_word_freq: row.get(4)?,
        })
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for WordRow {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(WordRow {
            words_rowid: row.get(0)?,
            lang_rowid: row.get(1)?,
            text: row.get(2)?,
            freq: row.get(3)?,
        })
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for SentenceMembershipRow {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(SentenceMembershipRow {
            sentence_memberships_rowid: row.get(0)?,
            sentence_rowid: row.get(1)?,
            word_rowid: row.get(2)?,
        })
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for SentenceMembershipWithTextEtc {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(SentenceMembershipWithTextEtc {
            sentence_memberships_rowid: row.get(0)?,
            sentence_rowid: row.get(1)?,
            word_rowid: row.get(2)?,
            word_text: row.get(3)?,
            word_freq: row.get(4)?,
            word_is_known: row.get(5)?,
        })
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for TranslationRow {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(TranslationRow {
            translations_rowid: row.get(0)?,
            target_lang_sentence_rowid: row.get(1)?,
            reference_lang_sentence_rowid: row.get(2)?,
        })
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for TranslationWithText {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(TranslationWithText {
            translations_rowid: row.get(0)?,
            target_lang_sentence_rowid: row.get(1)?,
            reference_lang_sentence_rowid: row.get(2)?,
            reference_lang_sentence_text: row.get(3)?,
        })
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for WordFrontierWithTranslation {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(WordFrontierWithTranslation {
            target_lang_sentence_rowid: row.get(0)?,
            target_lang_text: row.get(1)?,
            reference_lang_sentence_rowid: row.get(2)?,
            reference_lang_text: row.get(3)?,
        })
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for KnownWordRow {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(KnownWordRow {
            known_words_rowid: row.get(0)?,
            word_rowid: row.get(1)?,
        })
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for KnownWordWithText {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(KnownWordWithText {
            known_words_rowid: row.get(0)?,
            word_rowid: row.get(1)?,
            word_text: row.get(2)?,
        })
    }
}

pub struct CorpusDb {
    conn: rusqlite::Connection,
}

impl CorpusDb {
    pub fn open(db_p: impl AsRef<Path>) -> Result<CorpusDb> {
        let conn = rusqlite::Connection::open(db_p)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS langs (
                langs_rowid INTEGER PRIMARY KEY,
                short TEXT UNIQUE NOT NULL,
                long TEXT NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sentences (
                sentences_rowid INTEGER PRIMARY KEY,
                lang_rowid INTEGER NOT NULL,
                text TEXT UNIQUE NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS words (
                words_rowid INTEGER PRIMARY KEY,
                lang_rowid INTEGER NOT NULL,
                text TEXT NOT NULL,
                freq INTEGER NOT NULL DEFAULT 1,
                UNIQUE(lang_rowid, text)
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sentence_memberships (
                sentence_memberships_rowid INTEGER PRIMARY KEY,
                sentence_rowid INTEGER NOT NULL,
                word_rowid INTEGER NOT NULL,
                UNIQUE(sentence_rowid, word_rowid)
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS translations (
                translations_rowid INTEGER PRIMARY KEY,
                target_lang_sentence_rowid INTEGER NOT NULL,
                reference_lang_sentence_rowid INTEGER NOT NULL,
                UNIQUE(target_lang_sentence_rowid, reference_lang_sentence_rowid)
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS known_words (
                known_words_rowid INTEGER PRIMARY KEY,
                word_rowid INTEGER NOT NULL,
                UNIQUE(word_rowid)
            )",
            [],
        )?;
        Ok(CorpusDb { conn })
    }
    pub fn import_from_sentence_pairs_tsv(
        &mut self,
        target_lang: Lang,
        reference_lang: Lang,
        sentence_pairs_tsv_p: impl AsRef<Path>,
    ) -> Result<()> {
        let tx = self.conn.transaction()?;

        // Ensure target and reference Lang-s are inserted.
        {
            let mut insert_lang_stmt = tx.prepare(
                "INSERT INTO langs (short, long) VALUES (?1, ?2) ON CONFLICT(short) DO NOTHING",
            )?;
            insert_lang_stmt.execute(rusqlite::params![target_lang.short, target_lang.long])?;
            insert_lang_stmt
                .execute(rusqlite::params![reference_lang.short, reference_lang.long])?;
        }
        // Retrieve the target and reference lang rowid-s.
        let target_lang_rowid = tx.query_row::<i32, _, _>(
            "SELECT langs_rowid FROM langs WHERE short = (?1)",
            rusqlite::params![target_lang.short],
            |row| row.get(0),
        )?;
        let reference_lang_rowid = tx.query_row::<i32, _, _>(
            "SELECT langs_rowid FROM langs WHERE short = (?1)",
            rusqlite::params![reference_lang.short],
            |row| row.get(0),
        )?;

        // Read all sentences into memory.  This will help the processing time by using more memory.
        // Although not anywhere close to a single Chrome browser tab...
        // Also record all the translations during this pass, so no intermediate storage is needed.
        let sentence_row_v = {
            let mut insert_translation = tx.prepare("INSERT INTO translations (target_lang_sentence_rowid, reference_lang_sentence_rowid) VALUES (?1, ?2)")?;

            log::info!("importing from path {:?}", sentence_pairs_tsv_p.as_ref());
            // First, determine the number of lines in the file.
            let line_count =
                std::io::BufReader::new(std::fs::File::open(sentence_pairs_tsv_p.as_ref())?)
                    .lines()
                    .count();
            log::info!("file line_count is {}", line_count);
            // Allocate a sentence vector of the given capacity.
            let mut sentence_row_v = Vec::with_capacity(line_count);
            // Now iterate through the content itself.
            for (line_index, tsv_line_r) in
                std::io::BufReader::new(std::fs::File::open(sentence_pairs_tsv_p.as_ref())?)
                    .lines()
                    .enumerate()
            {
                let line_number = line_index + 1;
                let tsv_line = tsv_line_r?;
                let tsv_line_v: Vec<&str> = tsv_line.split('\t').collect();
                if tsv_line_v.len() != 4 {
                    log::warn!(
                        "parse error on line {}: expected 4 tab-separated values but found {}",
                        line_number,
                        tsv_line_v.len()
                    );
                    continue;
                }

                // Parse out the sentences in target and reference langs.
                let target_lang_sentence_rowid = match i32::from_str(tsv_line_v[0]) {
                    Ok(n) => n,
                    Err(e) => {
                        log::warn!(
                            "parse error in sentence_rowid {:?} on line {}; error was {}",
                            tsv_line_v[0],
                            line_number,
                            e
                        );
                        continue;
                    }
                };
                let target_lang_sentence_text = tsv_line_v[1];
                let reference_lang_sentence_rowid = match i32::from_str(tsv_line_v[2]) {
                    Ok(n) => n,
                    Err(e) => {
                        log::warn!(
                            "parse error in sentence_rowid {:?} on line {}; error was {}",
                            tsv_line_v[2],
                            line_number,
                            e
                        );
                        continue;
                    }
                };
                let reference_lang_sentence_text = tsv_line_v[3];
                // Store the sentences.
                sentence_row_v.push(SentenceRow {
                    sentences_rowid: target_lang_sentence_rowid,
                    lang_rowid: target_lang_rowid,
                    text: target_lang_sentence_text.into(),
                });
                sentence_row_v.push(SentenceRow {
                    sentences_rowid: reference_lang_sentence_rowid,
                    lang_rowid: reference_lang_rowid,
                    text: reference_lang_sentence_text.into(),
                });
                // Record the translation.
                insert_translation.execute(rusqlite::params![
                    target_lang_sentence_rowid,
                    reference_lang_sentence_rowid
                ])?;
            }

            sentence_row_v
        };

        // Form word_row_m and sentence_membership_sm.
        // TODO: Could use str here which would be views into the String-s in sentence_row_v, and not
        // allocate words_rowid yet.
        let mut word_row_m: HashMap<String, WordRow> = HashMap::new();
        let mut sentence_membership_sm: HashMap<i32, HashSet<i32>> = HashMap::new();
        {
            let mut words_rowid: i32 = 1;
            for sentence_row in sentence_row_v.iter() {
                // Create a word set for the sentence.
                let mut sentence_words_rowid_s: HashSet<i32> = HashSet::new();

                // Now also parse the sentence and gather words.
                for word_str in sentence_row.text.split_whitespace() {
                    // Clean punctuation off of the word.
                    let trim_pattern: &[char] = &['.', ',', '!', '?', '"', '\''];
                    let word_str = word_str
                        .trim_start_matches(trim_pattern)
                        .trim_end_matches(trim_pattern);
                    match word_row_m.get_mut(word_str) {
                        Some(word_row) => {
                            // If the word existed already, bump freq up by 1.
                            assert_eq!(word_row.text, word_str);
                            word_row.freq += 1;
                            // Ensure this word is added to sentence_words_rowid_s.
                            sentence_words_rowid_s.insert(word_row.words_rowid);
                        }
                        None => {
                            // If the word didn't already exist, add it with freq 1.
                            word_row_m.insert(
                                word_str.into(),
                                WordRow {
                                    words_rowid,
                                    lang_rowid: sentence_row.lang_rowid,
                                    text: word_str.into(),
                                    freq: 1,
                                },
                            );
                            // Ensure this word is added to sentence_words_rowid_s.
                            sentence_words_rowid_s.insert(words_rowid);
                            // Increment words_rowid for the next one.
                            words_rowid += 1;
                        }
                    }
                }

                // Record the word set.
                sentence_membership_sm.insert(sentence_row.sentences_rowid, sentence_words_rowid_s);
            }
        }

        // Insert sentences
        {
            // Prepare an SQL statement ahead of time to avoid its preparation overhead inside the loop.
            let mut insert_sentence = tx.prepare("INSERT INTO sentences (sentences_rowid, lang_rowid, text) VALUES (?1, ?2, ?3) ON CONFLICT DO NOTHING")?;
            for sentence_row in sentence_row_v.iter() {
                // TODO: Figure out how to do this more cleanly, e.g. with some From trait
                insert_sentence.execute(rusqlite::params![
                    sentence_row.sentences_rowid,
                    sentence_row.lang_rowid,
                    sentence_row.text,
                ])?;
            }
        }

        // Insert words
        {
            let mut insert_word = tx.prepare(
                "INSERT INTO words (words_rowid, lang_rowid, text, freq) VALUES (?1, ?2, ?3, ?4)",
            )?;
            for word_row in word_row_m.values() {
                // TODO: Figure out how to do this more cleanly, e.g. with a From trait
                insert_word.execute(rusqlite::params![
                    word_row.words_rowid,
                    word_row.lang_rowid,
                    word_row.text,
                    word_row.freq
                ])?;
            }
        }

        // Insert sentence memberships
        {
            let mut insert_sentence_membership = tx.prepare(
                "INSERT INTO sentence_memberships (sentence_rowid, word_rowid) VALUES (?1, ?2)",
            )?;
            for (sentence_rowid, word_rowid_s) in sentence_membership_sm.iter() {
                for word_rowid in word_rowid_s.iter() {
                    insert_sentence_membership
                        .execute(rusqlite::params![sentence_rowid, word_rowid])?;
                }
            }
        }

        tx.commit()?;

        Ok(())
    }
    // TODO: Maybe make one that optionally takes a transaction, in order to reduce duplication.
    pub fn langs_rowid_of(&self, lang_short: impl AsRef<str>) -> Result<i32> {
        Ok(self.conn.query_row::<i32, _, _>(
            "SELECT langs_rowid FROM langs WHERE short = (?1)",
            rusqlite::params![lang_short.as_ref()],
            |row| row.get(0),
        )?)
    }
    pub fn query_translation_with_text_v(
        &self,
        target_lang_sentence_rowid: i32,
    ) -> Result<Vec<TranslationWithText>> {
        let mut stmt = self.conn.prepare("
            -- Human-friendly query of translations
            SELECT translations.translations_rowid, translations.target_lang_sentence_rowid, translations.reference_lang_sentence_rowid, sentences.text
            FROM translations
            INNER JOIN sentences ON sentences.sentences_rowid = translations.reference_lang_sentence_rowid
            WHERE translations.target_lang_sentence_rowid = ?1
        ")?;
        let translation_with_text_v = stmt
            .query_map(
                rusqlite::params![target_lang_sentence_rowid],
                |row| TranslationWithText::try_from(row),
            )?
            .map(|translation_with_text_r| translation_with_text_r.unwrap())
            .collect();
        Ok(translation_with_text_v)
    }
    pub fn query_sentence_membership_with_text_etc_v(
        &self,
        sentence_rowid: i32,
    ) -> Result<Vec<SentenceMembershipWithTextEtc>> {
        let mut stmt = self.conn.prepare("
            -- Human-friendly query of sentence_memberships
            SELECT
			    sentence_memberships.sentence_memberships_rowid,
				sentence_memberships.sentence_rowid,
				sentence_memberships.word_rowid,
				words.text,
				words.freq,
				(sentence_memberships.word_rowid IN (SELECT known_words.word_rowid FROM known_words))
            FROM sentence_memberships
            INNER JOIN words ON words.words_rowid = sentence_memberships.word_rowid
            WHERE sentence_memberships.sentence_rowid = ?1
        ")?;
        let sentence_membership_with_text_etc_v = stmt
            .query_map(
                rusqlite::params![sentence_rowid],
                |row| SentenceMembershipWithTextEtc::try_from(row),
            )?
            .map(|sentence_membership_with_text_etc_r| sentence_membership_with_text_etc_r.unwrap())
            .collect();
        Ok(sentence_membership_with_text_etc_v)
    }
    pub fn query_known_word_with_text_v(
        &self,
        target_lang_rowid: i32,
    ) -> Result<Vec<KnownWordWithText>> {
        let mut stmt = self.conn.prepare("
            -- Human-friendly query of known words
            SELECT known_words.known_words_rowid, known_words.word_rowid, words.text
            FROM known_words
            INNER JOIN words ON words.words_rowid = known_words.word_rowid
            WHERE words.lang_rowid = ?1
        ")?;
        let known_word_with_text_v = stmt
            .query_map(
                rusqlite::params![target_lang_rowid],
                |row| KnownWordWithText::try_from(row),
            )?
            .map(|known_word_with_text_r| known_word_with_text_r.unwrap())
            .collect();
        Ok(known_word_with_text_v)
    }
    pub fn add_known_word(
        &self,
        word_rowid: i32,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO known_words (word_rowid) VALUES (?1) ON CONFLICT(word_rowid) DO NOTHING",
            [word_rowid],
        )?;
        Ok(())
    }
    pub fn remove_known_word(
        &self,
        word_rowid: i32,
    ) -> Result<()> {
        self.conn.execute(
            "DELETE FROM known_words WHERE word_rowid = ?1",
            [word_rowid],
        )?;
        Ok(())
    }
    pub fn query_word_frontier_v(
        &self,
        known_word_count_range: Range,
        target_lang_rowid: i32,
        order: Order,
    ) -> Result<Vec<WordFrontierMember>> {
        let ordering_str = match order {
            Order::Ascending => "ORDER BY unknown_word_freq ASC",
            Order::Descending => "ORDER BY unknown_word_freq DESC",
            Order::Unordered => "",
        };
        let mut stmt = self.conn.prepare(&format!("
            -- This selects sentence_rowid for sentences having a number of unknown words in a certain range.
            SELECT sentences.sentences_rowid, sentences.lang_rowid, sentences.text, (
                SELECT COUNT(*)
                FROM sentence_memberships
                WHERE sentence_memberships.sentence_rowid = sentences.sentences_rowid AND sentence_memberships.word_rowid NOT IN (SELECT known_words.word_rowid FROM known_words)
                GROUP BY sentence_memberships.sentence_rowid
                ORDER BY sentence_memberships.sentence_rowid
            ), (
                SELECT MIN(words.freq)
                FROM sentence_memberships
                INNER JOIN words ON words.words_rowid = sentence_memberships.word_rowid
                WHERE sentence_memberships.sentence_rowid = sentences.sentences_rowid AND sentence_memberships.word_rowid NOT IN (SELECT known_words.word_rowid FROM known_words)
                GROUP BY sentence_memberships.sentence_rowid
                ORDER BY sentence_memberships.sentence_rowid
            ) as unknown_word_freq
            FROM sentences
            WHERE (
                SELECT COUNT(*)
                FROM sentence_memberships
                WHERE sentence_memberships.sentence_rowid = sentences.sentences_rowid AND sentence_memberships.word_rowid NOT IN (SELECT known_words.word_rowid FROM known_words)
                GROUP BY sentence_memberships.sentence_rowid
                ORDER BY sentence_memberships.sentence_rowid
            ) BETWEEN ?1 AND ?2
            AND
            sentences.lang_rowid = ?3
            -- GROUP BY sentences.sentences_rowid
            {}
        ", ordering_str))?;
        let word_frontier_member_v = stmt
            .query_map(
                rusqlite::params![
                    known_word_count_range.0,
                    known_word_count_range.1,
                    target_lang_rowid
                ],
                |row| WordFrontierMember::try_from(row),
            )?
            .map(|word_frontier_member_r| word_frontier_member_r.unwrap())
            .collect();
        Ok(word_frontier_member_v)
    }
    pub fn query_word_frontier_with_translation_v(
        &self,
        known_word_count_range: Range,
        target_lang_rowid: i32,
        reference_lang_rowid: i32,
    ) -> Result<Vec<WordFrontierWithTranslation>> {
        let mut stmt = self.conn.prepare("
            -- This selects sentence_rowid for sentences having a number of unknown words in a certain range,
            -- and joins the translations of those sentences
            SELECT S1.sentences_rowid, S1.text, reference_lang_sentence_rowid, S2.text
            FROM sentences AS S1
            INNER JOIN translations ON translations.target_lang_sentence_rowid = S1.sentences_rowid
            INNER JOIN sentences AS S2 ON translations.reference_lang_sentence_rowid = S2.sentences_rowid
            WHERE (
                SELECT COUNT(*)
                FROM sentence_memberships
                WHERE S1.sentences_rowid = sentence_rowid AND word_rowid NOT IN (SELECT known_words_rowid FROM known_words)
                GROUP BY sentence_rowid
                ORDER BY sentence_rowid
            ) BETWEEN ?1 AND ?2
            AND
            S1.lang_rowid = ?3
            AND
            S2.lang_rowid = ?4
        ")?;
        let sentence_row_v = stmt
            .query_map(
                rusqlite::params![
                    known_word_count_range.0,
                    known_word_count_range.1,
                    target_lang_rowid,
                    reference_lang_rowid
                ],
                |row| WordFrontierWithTranslation::try_from(row),
            )?
            .map(|word_frontier_with_translation_r| word_frontier_with_translation_r.unwrap())
            .collect();
        Ok(sentence_row_v)
    }
}
