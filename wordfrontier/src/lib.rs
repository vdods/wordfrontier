mod corpus_db;
mod error;

pub use crate::{
    corpus_db::{
        CorpusDb, KnownWordWithText, Lang, OnConflict, Order, Range, SentenceMembershipWithTextEtc, SentenceRow,
        TranslationWithText, WordFrontierMember,
    },
    error::Error,
};

pub type Result<T> = std::result::Result<T, Error>;
