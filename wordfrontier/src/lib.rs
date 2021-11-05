#![allow(dead_code)] // TEMP HACK

mod corpus_db;
mod db_hub;
mod error;
mod langs_db;
mod translations_db;
mod user_db;

pub use crate::{
    corpus_db::{CorpusDb, CorpusPurpose, SentenceRow},
    db_hub::{DbHub, DbHubConfig, KnownWordWithText, SentenceMembershipWithTextEtc, TranslationWithText, WordFrontierMember},
    langs_db::{Lang, LangsDb, LangRow},
    translations_db::{TranslationsDb},
    user_db::{UserDb},
    error::Error,
};
pub(crate) use crate::langs_db::LANG_M;

/// See https://www.sqlite.org/lang_conflict.html -- note that OnConflict::Fail is the
/// default behavior if no "ON XXX" is specified in the INSERT SQL statement.
#[derive(Debug, Clone, Copy)]
pub enum OnConflict {
    Abort,
    Fail,
    Ignore,
    Replace,
    Rollback,
}

impl std::fmt::Display for OnConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            OnConflict::Abort => write!(f, "ABORT"),
            OnConflict::Fail => write!(f, "FAIL"),
            OnConflict::Ignore => write!(f, "IGNORE"),
            OnConflict::Replace => write!(f, "REPLACE"),
            OnConflict::Rollback => write!(f, "ROLLBACK"),
        }
    }
}

#[derive(Debug)]
pub enum Order {
    Ascending,
    Descending,
    Unordered,
}

// TODO: Use appropriate type with trait with comparison operators
pub struct Range(pub i32, pub i32);

pub type Result<T> = std::result::Result<T, Error>;
