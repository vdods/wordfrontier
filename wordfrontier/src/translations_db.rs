use crate::{OnConflict, LANG_M, Result};
use std::{convert::TryFrom, path::Path};

pub struct TranslationRow {
    pub translations_rowid: i32,
    pub target_lang_sentence_rowid: i32,
    pub reference_lang_sentence_rowid: i32,
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


pub struct TranslationsDb {
    target_lang_short: String,
    reference_lang_short: String,
    conn: rusqlite::Connection,
}

impl TranslationsDb {
    pub async fn create_and_populate_if_missing(target_lang_short: &str, reference_lang_short: &str, override_base_url_o: Option<&str>) -> Result<()> {
        let db_p = Self::db_path_from(target_lang_short, reference_lang_short)?;
        if !Path::new(&db_p).exists() {
            Self::open(target_lang_short, reference_lang_short)?.populate(override_base_url_o).await?;
        }
        Ok(())
    }
    pub fn attach(conn: &rusqlite::Connection, target_lang_short: &str, reference_lang_short: &str) -> Result<()> {
        let db_p = Self::db_path_from(target_lang_short, reference_lang_short)?;
        log::info!("TranslationsDb; attaching database");
        conn.execute("ATTACH DATABASE ?1 AS translations_db", rusqlite::params![db_p])?;
        Ok(())
    }
    pub fn db_path_from(target_lang_short: &str, reference_lang_short: &str) -> Result<String> {
        LANG_M.get(target_lang_short)
            .ok_or_else(|| anyhow::anyhow!("target_lang_short {:#?} not found", target_lang_short))?;
        LANG_M.get(reference_lang_short)
            .ok_or_else(|| anyhow::anyhow!("reference_lang_short {:#?} not found", reference_lang_short))?;
        Ok(format!("translations.target={}.reference={}.db", target_lang_short, reference_lang_short))
    }

    pub fn open(target_lang_short: &str, reference_lang_short: &str) -> Result<Self> {
        let db_p = Self::db_path_from(target_lang_short, reference_lang_short)?;
        log::info!("TranslationsDb; opening {:#?}", db_p);
        let conn = rusqlite::Connection::open(db_p)?;
        Ok(Self {
            target_lang_short: target_lang_short.into(),
            reference_lang_short: reference_lang_short.into(),
            conn,
        })
    }
    pub async fn populate(
        &mut self,
        override_base_url_o: Option<&str>,
    ) -> Result<()> {
        let tx = self.conn.transaction()?;

        let url = {
            let default_base_url = "https://downloads.tatoeba.org/exports/per_language";
            let base_url = override_base_url_o.unwrap_or(default_base_url);
            format!(
                "{}/{}/{}-{}_links.tsv.bz2",
                base_url,
                self.target_lang_short,
                self.target_lang_short,
                self.reference_lang_short,
            )
        };

        log::info!("TranslationsDb; populating from {:#?}", url);

        // Create the table(s).
        tx.execute(
            "CREATE TABLE IF NOT EXISTS translations (
                translations_rowid INTEGER PRIMARY KEY,
                target_lang_sentence_rowid INTEGER NOT NULL,
                reference_lang_sentence_rowid INTEGER NOT NULL,
                UNIQUE(target_lang_sentence_rowid, reference_lang_sentence_rowid)
            )",
            [],
        )?;

        // Download and ingest the content
        {
            let mut insert_translation = tx.prepare(
                &format!("INSERT OR {} INTO translations (target_lang_sentence_rowid, reference_lang_sentence_rowid) VALUES (?1, ?2)", OnConflict::Ignore)
            )?;

            let default_base_url = "https://downloads.tatoeba.org/exports/per_language";
            let base_url = override_base_url_o.unwrap_or(default_base_url);

            // TODO: Figure out how to do this in a streaming way
            let compressed_bytes =
                reqwest::get(format!("{}/{}/{}-{}_links.tsv.bz2", base_url, self.target_lang_short, self.target_lang_short, self.reference_lang_short))
                .await?
                // TODO: Streaming into bzip2 decompression
                .bytes()
                .await?;
            let mut bz2_decoder = bzip2::bufread::BzDecoder::new(compressed_bytes.as_ref());
            // TODO: Try to pre-allocate capacity
            let mut translations_tsv_string = String::new();
            use std::io::Read;
            bz2_decoder.read_to_string(&mut translations_tsv_string)?;

//             log::debug!("translations_tsv_string:\n{}", translations_tsv_string);

            for (line_index, translation_tsv_line) in translations_tsv_string.split('\n').enumerate() {
                let line_number = line_index + 1;
                let mut tsv_split = translation_tsv_line.split('\t');
                // TODO: Factor this
                let target_lang_sentence_rowid = match tsv_split.next() {
                    Some(target_lang_sentence_rowid_str) => match str::parse::<i32>(target_lang_sentence_rowid_str) {
                        Ok(target_lang_sentence_rowid) => target_lang_sentence_rowid,
                        Err(e) => {
                            log::warn!("Parse error {} in translations TSV data on line {}; expected integer rowid value, but got {:#?}.  Ignoring this line.", e, line_number, target_lang_sentence_rowid_str);
                            continue;
                        }
                    },
                    None => {
                        log::warn!("Malformed translations TSV data on line {}; expected integer rowid value, but found nothing.  Ignoring this line.", line_number);
                        continue;
                    },
                };
                let reference_lang_sentence_rowid = match tsv_split.next() {
                    Some(reference_lang_sentence_rowid_str) => match str::parse::<i32>(reference_lang_sentence_rowid_str) {
                        Ok(reference_lang_sentence_rowid) => reference_lang_sentence_rowid,
                        Err(e) => {
                            log::warn!("Parse error {} in translations TSV data on line {}; expected integer rowid value, but got {:#?}.  Ignoring this line.", e, line_number, reference_lang_sentence_rowid_str);
                            continue;
                        }
                    },
                    None => {
                        log::warn!("Malformed translations TSV data on line {}; expected integer rowid value, but found nothing.  Ignoring this line.", line_number);
                        continue;
                    },
                };
                match tsv_split.next() {
                    Some(s) => {
                        log::warn!("Unexpected third value {:#?} in translations TSV data on line {}.  Ignoring this line.", s, line_number);
                        continue;
                    },
                    None => {
                    },
                }

                // Record the translation.
                insert_translation.execute(rusqlite::params![
                    target_lang_sentence_rowid,
                    reference_lang_sentence_rowid,
                ])?;
            }
        }

        tx.commit()?;

        Ok(())
    }
}
