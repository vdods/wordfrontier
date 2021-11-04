use wordfrontier::{CorpusDb, OnConflict, Order, Range, Result};

// use wordfrontier::Lang;
//
// #[test]
// fn test_corpus_db_import() -> Result<()> {
//     let _ = env_logger::try_init();
//
//     let mut corpus_db = CorpusDb::open("thingy.db")?;
//     corpus_db.import_from_sentence_pairs_tsv(
//         Lang {
//             short: "deu".into(),
//             long: "Deutsch".into(),
//         },
//         Lang {
//             short: "eng".into(),
//             long: "English".into(),
//         },
//         // "downloads/Sentence pairs in German-English - 2021-08-29.short.tsv",
//         "downloads/Sentence pairs in German-English - 2021-08-29.tsv",
//         OnConflict::Ignore,
//     )?;
//     Ok(())
// }

#[test]
fn test_corpus_db_query_word_frontier() -> Result<()> {
    let _ = env_logger::try_init();

    let corpus_db = CorpusDb::open("thingy.db")?;
    let word_frontier_v =
        corpus_db.query_word_frontier_v(Range(1, 1), corpus_db.langs_rowid_of("deu")?, Order::Ascending)?;
    log::debug!("word_frontier_v: {:#?}", word_frontier_v);
    log::debug!("word_frontier_v.len(): {:#?}", word_frontier_v.len());
    Ok(())
}

#[test]
fn test_corpus_db_x_query_word_frontier_with_translations() -> Result<()> {
    let _ = env_logger::try_init();

    let corpus_db = CorpusDb::open("thingy.db")?;
    let word_frontier_with_translation_v = corpus_db.query_word_frontier_with_translation_v(
        Range(2, 2),
        corpus_db.langs_rowid_of("deu")?,
        corpus_db.langs_rowid_of("eng")?,
    )?;
    log::debug!("word_frontier_with_translation_v.len(): {:#?}", word_frontier_with_translation_v.len());
    for word_frontier_with_translation in word_frontier_with_translation_v.iter() {
        log::debug!(
            "{:?} <=> {:?}",
            word_frontier_with_translation.target_lang_text,
            word_frontier_with_translation.reference_lang_text
        );
    }
    log::debug!(
        "word_frontier_with_translation_v.len(): {:#?}",
        word_frontier_with_translation_v.len()
    );
    Ok(())
}
