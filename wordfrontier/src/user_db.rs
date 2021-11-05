use crate::Result;
use std::{convert::TryFrom, path::Path};

pub struct KnownWordRow {
    pub known_words_rowid: i32,
    pub word_rowid: i32,
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

pub struct UserDb {
    conn: rusqlite::Connection,
}

impl UserDb {
    pub fn create_and_populate_if_missing() -> Result<()> {
        if !Path::new(Self::db_path()).exists() {
            Self::open()?.populate()?
        }
        Ok(())
    }
    pub fn attach(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute("ATTACH DATABASE ?1 AS user_db", rusqlite::params![Self::db_path()])?;
        Ok(())
    }
    pub fn db_path() -> &'static str {
        "user.db"
    }

    pub fn open() -> Result<Self> {
        let conn = rusqlite::Connection::open(Self::db_path())?;
        Ok(Self { conn })
    }
    pub fn populate(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS known_words (
                known_words_rowid INTEGER PRIMARY KEY,
                lang_rowid INTEGER NOT NULL,
                word_rowid INTEGER NOT NULL,
                UNIQUE(lang_rowid, word_rowid)
            )",
            [],
        )?;
        Ok(())
    }
}
