use crate::{CorpusDb, CorpusPurpose, Lang, LangsDb, Order, LANG_M, Range, Result, TranslationsDb, UserDb};
use std::convert::TryFrom;

pub struct SentenceMembershipWithTextEtc {
    pub sentence_memberships_rowid: i32,
    pub sentence_rowid: i32,
    pub word_rowid: i32,
    pub word_text: String,
    pub word_freq: i32,
    pub word_is_known: bool,
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

#[derive(Debug)]
pub struct KnownWordWithText {
    pub known_words_rowid: i32,
    pub lang_rowid: i32,
    pub word_rowid: i32,
    pub word_text: String,
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for KnownWordWithText {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row<'stmt>) -> std::result::Result<Self, Self::Error> {
        Ok(KnownWordWithText {
            known_words_rowid: row.get(0)?,
            lang_rowid: row.get(1)?,
            word_rowid: row.get(2)?,
            word_text: row.get(3)?,
        })
    }
}

pub struct TranslationWithText {
    pub translations_rowid: i32,
    pub target_lang_sentence_rowid: i32,
    pub reference_lang_sentence_rowid: i32,
    pub reference_lang_sentence_text: String,
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

#[derive(Debug)]
pub struct WordFrontierMember {
    pub sentences_rowid: i32,
    pub lang_rowid: i32,
    pub text: String,
    pub unknown_word_count: i32,
    pub unknown_word_freq: i32,
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

#[derive(Debug)]
pub struct DbHubConfig {
    target_lang: Lang,
    reference_lang: Lang,
    // NOTE: This maybe doesn't belong here, since this is an implementation detail of
    // retrieving the content, not simply loading the DBs.
    override_base_url_o: Option<String>,
}

impl DbHubConfig {
    pub fn new(
        target_lang_short: &str,
        reference_lang_short: &str,
        override_base_url_o: Option<String>,
    ) -> Result<DbHubConfig> {
        let target_lang = LANG_M.get(target_lang_short)
            .ok_or_else(
                || anyhow::anyhow!("target_lang_short {:#?} not found", target_lang_short)
            )?;
        let reference_lang = LANG_M.get(reference_lang_short)
            .ok_or_else(
                || anyhow::anyhow!("reference_lang_short {:#?} not found", reference_lang_short)
            )?;
        Ok(DbHubConfig {
            target_lang: target_lang.clone(),
            reference_lang: reference_lang.clone(),
            override_base_url_o,
        })
    }
}

// This opens several DBs via ATTACH statements, and orchestrates their interaction.
pub struct DbHub {
    db_hub_config: DbHubConfig,
    conn: rusqlite::Connection,
    target_lang_rowid: i32,
    reference_lang_rowid: i32,
}

impl DbHub {
    pub async fn create_and_populate_missing_databases(db_hub_config: &DbHubConfig) -> Result<()> {
        LangsDb::create_and_populate_if_missing()?;
        let (target_lang_row, reference_lang_row) = {
            let langs_db = LangsDb::open()?;
            let target_lang_row = langs_db.query_lang_row(&db_hub_config.target_lang.short)?;
            let reference_lang_row = langs_db.query_lang_row(&db_hub_config.reference_lang.short)?;
            (target_lang_row, reference_lang_row)
        };
        UserDb::create_and_populate_if_missing()?;
        CorpusDb::create_and_populate_if_missing(
            target_lang_row,
            db_hub_config
                .override_base_url_o
                .as_ref()
                .map(|s| s.as_str())
        ).await?;
        CorpusDb::create_and_populate_if_missing(
            reference_lang_row,
            db_hub_config
                .override_base_url_o
                .as_ref()
                .map(|s| s.as_str()),
        ).await?;
        TranslationsDb::create_and_populate_if_missing(
            &db_hub_config.target_lang.short,
            &db_hub_config.reference_lang.short,
            db_hub_config
                .override_base_url_o
                .as_ref()
                .map(|s| s.as_str()),
        ).await?;
        Ok(())
    }
    pub fn from_config(db_hub_config: DbHubConfig) -> Result<DbHub> {
        log::debug!("DbHub::from_config({:#?})", db_hub_config);

        // TODO: Is opening an in-memory DB and attaching the file-backed ones a dumb idea?
        let conn = rusqlite::Connection::open(":memory:")?;

        LangsDb::attach(&conn)?;
        UserDb::attach(&conn)?;
        CorpusDb::attach(&conn, db_hub_config.target_lang.short, CorpusPurpose::TargetLang)?;
        CorpusDb::attach(&conn, db_hub_config.reference_lang.short, CorpusPurpose::ReferenceLang)?;
        TranslationsDb::attach(&conn, db_hub_config.target_lang.short, db_hub_config.reference_lang.short)?;

        let target_lang_rowid = Self::query_langs_rowid(&conn, &db_hub_config.target_lang.short)?;
        let reference_lang_rowid = Self::query_langs_rowid(&conn, &db_hub_config.reference_lang.short)?;

        Ok(DbHub { db_hub_config, conn, target_lang_rowid, reference_lang_rowid })
    }

    // TODO: Maybe make one that optionally takes a transaction, in order to reduce duplication.
    fn query_langs_rowid(conn: &rusqlite::Connection, lang_short: &str) -> Result<i32> {
        Ok(conn.query_row::<i32, _, _>(
            "SELECT langs_rowid FROM langs_db.langs WHERE short = ?1",
            rusqlite::params![lang_short],
            |row| row.get(0),
        )?)
    }
    pub fn query_known_word_with_text_v(&self) -> Result<Vec<KnownWordWithText>> {
        let mut stmt = self.conn.prepare("
            -- Human-friendly query of known words
            SELECT
                user_db.known_words.known_words_rowid,
                user_db.known_words.lang_rowid,
                user_db.known_words.word_rowid,
                target_corpus_db.words.text
            FROM user_db.known_words
            INNER JOIN target_corpus_db.words ON target_corpus_db.words.words_rowid = user_db.known_words.word_rowid
            WHERE
                target_corpus_db.words.lang_rowid = ?1
                AND
                user_db.known_words.lang_rowid = ?1
        ")?;
        let known_word_with_text_v = stmt
            .query_map(
                rusqlite::params![self.target_lang_rowid],
                |row| KnownWordWithText::try_from(row),
            )?
            .map(|known_word_with_text_r| known_word_with_text_r.unwrap())
            .collect();
        Ok(known_word_with_text_v)
    }
    pub fn query_translation_with_text_v(
        &self,
        target_lang_sentence_rowid: i32,
    ) -> Result<Vec<TranslationWithText>> {
        let mut stmt = self.conn.prepare("
            -- Human-friendly query of translations
            SELECT
                translations_db.translations.translations_rowid,
                translations_db.translations.target_lang_sentence_rowid,
                translations_db.translations.reference_lang_sentence_rowid,
                reference_corpus_db.sentences.text
            FROM translations_db.translations
            INNER JOIN
                reference_corpus_db.sentences
                ON
                reference_corpus_db.sentences.sentences_rowid = translations_db.translations.reference_lang_sentence_rowid
            WHERE translations_db.translations.target_lang_sentence_rowid = ?1
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
                target_corpus_db.sentence_memberships.sentence_memberships_rowid,
                target_corpus_db.sentence_memberships.sentence_rowid,
                target_corpus_db.sentence_memberships.word_rowid,
                target_corpus_db.words.text,
                target_corpus_db.words.freq,
                (
                    target_corpus_db.sentence_memberships.word_rowid
                    IN
                    (
                        SELECT user_db.known_words.word_rowid
                        FROM user_db.known_words
                        WHERE user_db.known_words.lang_rowid = ?1
                    )
                ) AS word_is_known
            FROM target_corpus_db.sentence_memberships
            INNER JOIN target_corpus_db.words ON target_corpus_db.words.words_rowid = target_corpus_db.sentence_memberships.word_rowid
            WHERE target_corpus_db.sentence_memberships.sentence_rowid = ?2
            ORDER BY word_is_known ASC
        ")?;
        let sentence_membership_with_text_etc_v = stmt
            .query_map(
                rusqlite::params![self.target_lang_rowid, sentence_rowid],
                |row| SentenceMembershipWithTextEtc::try_from(row),
            )?
            .map(|sentence_membership_with_text_etc_r| sentence_membership_with_text_etc_r.unwrap())
            .collect();
        Ok(sentence_membership_with_text_etc_v)
    }
    pub fn query_word_frontier_v(
        &self,
        known_word_count_range: Range,
        order: Order,
    ) -> Result<Vec<WordFrontierMember>> {
        let ordering_str = match order {
            Order::Ascending => "ORDER BY unknown_word_freq ASC",
            Order::Descending => "ORDER BY unknown_word_freq DESC",
            Order::Unordered => "",
        };
        let mut stmt = self.conn.prepare(&format!("
            -- This selects sentence_rowid for sentences having a number of unknown words in a certain range.
            SELECT
                target_corpus_db.sentences.sentences_rowid,
                target_corpus_db.sentences.lang_rowid,
                target_corpus_db.sentences.text,
                (
                    SELECT COUNT(*)
                    FROM target_corpus_db.sentence_memberships
                    WHERE
                        target_corpus_db.sentence_memberships.sentence_rowid = target_corpus_db.sentences.sentences_rowid
                        AND
                        target_corpus_db.sentence_memberships.word_rowid NOT IN (
                            SELECT user_db.known_words.word_rowid
                            FROM user_db.known_words
                            WHERE user_db.known_words.lang_rowid = ?1
                        )
                    GROUP BY target_corpus_db.sentence_memberships.sentence_rowid
                    ORDER BY target_corpus_db.sentence_memberships.sentence_rowid
                ),
                (
                    SELECT MIN(target_corpus_db.words.freq)
                    FROM target_corpus_db.sentence_memberships
                    INNER JOIN target_corpus_db.words ON target_corpus_db.words.words_rowid = target_corpus_db.sentence_memberships.word_rowid
                    WHERE
                        target_corpus_db.sentence_memberships.sentence_rowid = target_corpus_db.sentences.sentences_rowid
                        AND
                        target_corpus_db.sentence_memberships.word_rowid NOT IN (
                            SELECT user_db.known_words.word_rowid
                            FROM user_db.known_words
                            WHERE user_db.known_words.lang_rowid = ?1
                        )
                    GROUP BY target_corpus_db.sentence_memberships.sentence_rowid
                    ORDER BY target_corpus_db.sentence_memberships.sentence_rowid
                ) as unknown_word_freq
            FROM target_corpus_db.sentences
            WHERE
                target_corpus_db.sentences.sentences_rowid IN (
                    SELECT target_lang_sentence_rowid FROM translations_db.translations
                )
                AND
                (
                    SELECT COUNT(*)
                    FROM target_corpus_db.sentence_memberships
                    WHERE
                        target_corpus_db.sentence_memberships.sentence_rowid = target_corpus_db.sentences.sentences_rowid
                        AND
                        target_corpus_db.sentence_memberships.word_rowid NOT IN (
                            SELECT user_db.known_words.word_rowid
                            FROM user_db.known_words
                            WHERE user_db.known_words.lang_rowid = ?1
                        )
                    GROUP BY target_corpus_db.sentence_memberships.sentence_rowid
                    ORDER BY target_corpus_db.sentence_memberships.sentence_rowid
                ) BETWEEN ?2 AND ?3
                AND
                target_corpus_db.sentences.lang_rowid = ?1
            -- GROUP BY target_corpus_db.sentences.sentences_rowid
            {}
        ", ordering_str))?;
        let word_frontier_member_v = stmt
            .query_map(
                rusqlite::params![
                    self.target_lang_rowid,
                    known_word_count_range.0,
                    known_word_count_range.1,
                ],
                |row| WordFrontierMember::try_from(row),
            )?
            .map(|word_frontier_member_r| word_frontier_member_r.unwrap())
            .collect();
        Ok(word_frontier_member_v)
    }
    pub fn add_known_word(&self, word_rowid: i32) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO user_db.known_words (lang_rowid, word_rowid) VALUES (?1, ?2)",
            [self.target_lang_rowid, word_rowid],
        )?;
        Ok(())
    }
    pub fn remove_known_word(&self, word_rowid: i32) -> Result<()> {
        self.conn.execute(
            "DELETE FROM user_db.known_words WHERE lang_rowid = ?1 AND word_rowid = ?2",
            [self.target_lang_rowid, word_rowid],
        )?;
        Ok(())
    }
}
