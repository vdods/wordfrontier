use wordfrontier::{CorpusDb, DbHub, DbHubConfig, LangsDb, Order, Range, Result, TranslationsDb, UserDb};

#[tokio::test]
#[serial_test::serial]
async fn test_corpus_db_create_and_populate_from_download() -> Result<()> {
    let _ = env_logger::try_init();

    // TODO: Ensure the content is downloaded into a cached dir
    // TODO: Spin up an HTTP server here to serve the downloaded content

    let langs_db = LangsDb::open()?;
    let lang_row = langs_db.query_lang_row("deu")?;

    let mut corpus_db = CorpusDb::open(lang_row)?;
    corpus_db.populate(Some("http://localhost:7000")).await?;

    Ok(())
}

#[test]
#[serial_test::serial]
fn test_db_hub_query_word_frontier() -> Result<()> {
    let _ = env_logger::try_init();

    let target_lang_short = "deu";
    let reference_lang_short = "eng";

    let db_hub = DbHub::from_config(DbHubConfig::new(target_lang_short, reference_lang_short, None)?)?;
    let word_frontier_v = db_hub.query_word_frontier_v(
        Range(1, 1),
        Order::Ascending,
    )?;
    log::trace!("word_frontier_v: {:#?}", word_frontier_v);
    log::debug!("word_frontier_v.len(): {:#?}", word_frontier_v.len());
    Ok(())
}

#[test]
#[serial_test::serial]
fn test_db_hub_query_known_word_with_text() -> Result<()> {
    let _ = env_logger::try_init();

    let target_lang_short = "deu";
    let reference_lang_short = "eng";

    let db_hub = DbHub::from_config(DbHubConfig::new(target_lang_short, reference_lang_short, None)?)?;
    let known_word_with_text_v = db_hub.query_known_word_with_text_v()?;
    log::trace!("known_word_with_text_v: {:#?}", known_word_with_text_v);
    log::debug!("known_word_with_text_v.len(): {:#?}", known_word_with_text_v.len());
    Ok(())
}

#[test]
#[serial_test::serial]
fn test_langs_db_create_and_populate() -> Result<()> {
    let _ = env_logger::try_init();

    LangsDb::create_and_populate_if_missing()?;

    let langs_db = LangsDb::open()?;
    let lang_row = langs_db.query_lang_row("deu")?;
    log::debug!("lang_row: {:#?}", lang_row);
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_translations_db_create_and_populate_from_download() -> Result<()> {
    let _ = env_logger::try_init();

    // TODO: Ensure the content is downloaded into a cached dir
    // TODO: Spin up an HTTP server here to serve the downloaded content

    let target_lang_short = "deu";
    let reference_lang_short = "eng";
    let mut translations_db = TranslationsDb::open(target_lang_short, reference_lang_short)?;
    translations_db.populate(Some("http://localhost:7000")).await?;

    Ok(())
}

#[test]
#[serial_test::serial]
fn test_user_db_create() -> Result<()> {
    let _ = env_logger::try_init();

    UserDb::create_and_populate_if_missing()?;
    Ok(())
}
