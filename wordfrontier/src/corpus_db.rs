use crate::{LANG_M, LangRow, OnConflict, Result};
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    path::Path,
};

#[derive(Debug)]
pub struct SentenceRow {
    pub sentences_rowid: i32,
    pub lang_rowid: i32,
    pub text: String,
}

impl SentenceRow {
    pub fn from_tsv(tsv: &str, lang_rowid: i32) -> Result<Self> {
        let mut tsv_split = tsv.split('\t');
        match (
            tsv_split.next(),
            tsv_split.next(),
            tsv_split.next(),
            tsv_split.next(),
        ) {
            // We expect exactly 3 tab-separated strings (None indicates the end of strings)
            (Some(sentences_rowid_str), Some(_lang_short), Some(text), None) => {
                let sentences_rowid = str::parse::<i32>(sentences_rowid_str).or_else(
                    |e| Err(anyhow::anyhow!(
                        "Parse error {} in translations TSV data; expected integer rowid value, but got {:#?}",
                        e,  sentences_rowid_str
                    ))
                )?;
                Ok(SentenceRow { sentences_rowid, lang_rowid, text: text.into() })
            },
            // Anything else is an error.
            _ => Err(anyhow::anyhow!("Malformed sentences TSV data {:#?}", tsv))?,
        }
    }
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

#[derive(Debug)]
pub struct WordFrontierWithTranslation {
    pub target_lang_sentence_rowid: i32,
    pub target_lang_text: String,
    pub reference_lang_sentence_rowid: i32,
    pub reference_lang_text: String,
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

#[derive(Debug, Clone, Copy)]
pub enum CorpusPurpose {
    TargetLang,
    ReferenceLang,
}

impl CorpusPurpose {
    pub fn database_name(self) -> &'static str {
        match self {
            CorpusPurpose::TargetLang => "target_corpus_db",
            CorpusPurpose::ReferenceLang => "reference_corpus_db",
        }
    }
}

pub struct CorpusDb {
    lang_row: LangRow,
    conn: rusqlite::Connection,
}

impl CorpusDb {
    pub async fn create_and_populate_if_missing(lang_row: LangRow, override_base_url_o: Option<&str>) -> Result<()> {
        let db_p = Self::db_path_from(&lang_row.short)?;
        if !Path::new(&db_p).exists() {
            Self::open(lang_row)?.populate(override_base_url_o).await?;
        }
        Ok(())
    }
    pub fn attach(conn: &rusqlite::Connection, lang_short: &str, corpus_purpose: CorpusPurpose) -> Result<()> {
        let db_p = Self::db_path_from(lang_short)?;
        conn.execute("ATTACH DATABASE ?1 AS ?2", rusqlite::params![db_p, corpus_purpose.database_name()])?;
        Ok(())
    }
    pub fn db_path_from(lang_short: &str) -> Result<String> {
        LANG_M.get(lang_short)
            .ok_or_else(|| anyhow::anyhow!("lang_short {:#?} not found", lang_short))?;
        Ok(format!("corpus.lang={}.db", lang_short))
    }

    pub fn open(lang_row: LangRow) -> Result<Self> {
        let db_p = Self::db_path_from(&lang_row.short)?;
        let conn = rusqlite::Connection::open(db_p)?;
        Ok(Self { lang_row, conn })
    }
    pub async fn populate(
        &mut self,
        override_base_url_o: Option<&str>,
    ) -> Result<()> {
        let tx = self.conn.transaction()?;

        // Create all the tables
        tx.execute(
            "CREATE TABLE IF NOT EXISTS sentences (
                sentences_rowid INTEGER PRIMARY KEY,
                lang_rowid INTEGER NOT NULL,
                text TEXT UNIQUE NOT NULL
            )",
            [],
        )?;
        tx.execute(
            "CREATE TABLE IF NOT EXISTS words (
                words_rowid INTEGER PRIMARY KEY,
                lang_rowid INTEGER NOT NULL,
                text TEXT NOT NULL,
                freq INTEGER NOT NULL DEFAULT 1,
                UNIQUE(lang_rowid, text)
            )",
            [],
        )?;
        tx.execute(
            "CREATE TABLE IF NOT EXISTS sentence_memberships (
                sentence_memberships_rowid INTEGER PRIMARY KEY,
                sentence_rowid INTEGER NOT NULL,
                word_rowid INTEGER NOT NULL,
                UNIQUE(sentence_rowid, word_rowid)
            )",
            [],
        )?;

        // Now download and ingest the content

        let default_base_url = "https://downloads.tatoeba.org/exports/per_language";
        let base_url = override_base_url_o.unwrap_or(default_base_url);

        // TODO: Figure out how to do this in a streaming way
        let sentences_tsv_string = {
            let compressed_bytes =
                reqwest::get(format!("{}/{}/{}_sentences.tsv.bz2", base_url, self.lang_row.short, self.lang_row.short))
                .await?
                // TODO: Streaming into bzip2 decompression
                .bytes()
                .await?;
            let mut bz2_decoder = bzip2::bufread::BzDecoder::new(compressed_bytes.as_ref());
            // TODO: Try to pre-allocate capacity
            let mut sentences_tsv_string = String::new();
            use std::io::Read;
            bz2_decoder.read_to_string(&mut sentences_tsv_string)?;
            sentences_tsv_string
        };
        let line_count = sentences_tsv_string.split('\n').count();
        log::debug!("sentences TSV data had {} lines", line_count);

        let sentence_row_v = {
            // Allocate a sentence vector of the given capacity.
            let mut sentence_row_v = Vec::with_capacity(line_count);
            // Now iterate through the content itself.
            for (line_index, sentence_tsv_line) in sentences_tsv_string.split('\n').enumerate() {
                let line_number = line_index + 1;
                let sentence_row = match SentenceRow::from_tsv(sentence_tsv_line, self.lang_row.langs_rowid) {
                    Ok(sentence_row) => sentence_row,
                    Err(e) => {
                        log::warn!("On line {}, {}.  Ignoring this line.", line_number, e);
                        continue;
                    },
                };
                sentence_row_v.push(sentence_row);
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
                    let trim_pattern: &[char] = &['.', ',', '!', '¡', '?', '¿', '"', '\''];
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

        let on_conflict = OnConflict::Ignore;

        // Insert sentences
        {
            // Prepare an SQL statement ahead of time to avoid its preparation overhead inside the loop.
            let mut insert_sentence = tx.prepare(
                &format!("INSERT OR {} INTO sentences (sentences_rowid, lang_rowid, text) VALUES (?1, ?2, ?3)", on_conflict)
            )?;
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
                &format!("INSERT OR {} INTO words (words_rowid, lang_rowid, text, freq) VALUES (?1, ?2, ?3, ?4)", on_conflict),
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
                &format!("INSERT OR {} INTO sentence_memberships (sentence_rowid, word_rowid) VALUES (?1, ?2)", on_conflict),
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
}
