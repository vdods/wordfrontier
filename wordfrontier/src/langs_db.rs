use crate::{OnConflict, Result};
use std::{convert::TryFrom, path::Path};

#[derive(Debug, Clone)]
pub struct Lang {
    pub short: &'static str,
    pub long_english: &'static str,
    pub long_native: &'static str,
}

#[derive(Debug, Clone)]
pub struct LangRow {
    pub langs_rowid: i32,
    // TODO -- change to short_name
    pub short: String,
    // TODO -- change to long_name_english, long_name_native
    pub long: String,
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

pub struct LangsDb {
    conn: rusqlite::Connection,
}

impl LangsDb {
    pub fn create_and_populate_if_missing() -> Result<()> {
        if !Path::new(Self::db_path()).exists() {
            Self::open()?.populate()?
        }
        Ok(())
    }
    pub fn attach(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute("ATTACH DATABASE ?1 AS langs_db", rusqlite::params![Self::db_path()])?;
        Ok(())
    }
    pub fn db_path() -> &'static str {
        "langs.db"
    }

    pub fn open() -> Result<Self> {
        let conn = rusqlite::Connection::open(Self::db_path())?;
        Ok(Self { conn })
    }
    pub fn populate(&mut self) -> Result<()> {
        let tx = self.conn.transaction()?;

        tx.execute(
            "CREATE TABLE IF NOT EXISTS langs (
                langs_rowid INTEGER PRIMARY KEY,
                short TEXT UNIQUE NOT NULL,
                long TEXT NOT NULL
            )",
            [],
        )?;

        // Import all the langs -- TODO: Somehow retrieve from tatoeba.org instead
        // TODO: Somehow retrieve the long names also.
        {
            let mut insert_lang_stmt = tx.prepare(
                // This could reasonably be OnConflict::Replace, to ingest updates.
                &format!("INSERT OR {} INTO langs (short, long) VALUES (?1, ?2)", OnConflict::Ignore)
            )?;
            for (_, lang) in LANG_M.iter() {
                log::debug!("inserting {:#?}", lang);
                insert_lang_stmt.execute(rusqlite::params![lang.short, lang.long_native])?;
            }
        }
        tx.commit()?;

        Ok(())
    }
    pub fn query_lang_row(&self, lang_short: &str) -> Result<LangRow> {
        Ok(self.conn.query_row(
            "SELECT langs_rowid, short, long FROM langs WHERE short = ?1",
            rusqlite::params![lang_short],
            |row| LangRow::try_from(row),
        )?)
    }
}

lazy_static::lazy_static! {
    // This list was retrieved by hand on 2021.11.03 from the dir names at
    // https://downloads.tatoeba.org/exports/per_language/
    // TODO: Somehow retrieve this from tatoeba.org
    pub(crate) static ref LANG_M: std::collections::BTreeMap<&'static str, Lang> = maplit::btreemap! {
        "abk" => Lang { short: "abk", long_english: "", long_native: "", },
        "abq" => Lang { short: "abq", long_english: "", long_native: "", },
        "acm" => Lang { short: "acm", long_english: "", long_native: "", },
        "ady" => Lang { short: "ady", long_english: "", long_native: "", },
        "afb" => Lang { short: "afb", long_english: "", long_native: "", },
        "afh" => Lang { short: "afh", long_english: "", long_native: "", },
        "afr" => Lang { short: "afr", long_english: "", long_native: "", },
        "aii" => Lang { short: "aii", long_english: "", long_native: "", },
        "ain" => Lang { short: "ain", long_english: "", long_native: "", },
        "ajp" => Lang { short: "ajp", long_english: "", long_native: "", },
        "akl" => Lang { short: "akl", long_english: "", long_native: "", },
        "aln" => Lang { short: "aln", long_english: "", long_native: "", },
        "alt" => Lang { short: "alt", long_english: "", long_native: "", },
        "amh" => Lang { short: "amh", long_english: "", long_native: "", },
        "ang" => Lang { short: "ang", long_english: "", long_native: "", },
        "aoz" => Lang { short: "aoz", long_english: "", long_native: "", },
        "apc" => Lang { short: "apc", long_english: "", long_native: "", },
        "ara" => Lang { short: "ara", long_english: "", long_native: "", },
        "arg" => Lang { short: "arg", long_english: "", long_native: "", },
        "arq" => Lang { short: "arq", long_english: "", long_native: "", },
        "ary" => Lang { short: "ary", long_english: "", long_native: "", },
        "arz" => Lang { short: "arz", long_english: "", long_native: "", },
        "asm" => Lang { short: "asm", long_english: "", long_native: "", },
        "ast" => Lang { short: "ast", long_english: "", long_native: "", },
        "ava" => Lang { short: "ava", long_english: "", long_native: "", },
        "avk" => Lang { short: "avk", long_english: "", long_native: "", },
        "awa" => Lang { short: "awa", long_english: "", long_native: "", },
        "ayl" => Lang { short: "ayl", long_english: "", long_native: "", },
        "aym" => Lang { short: "aym", long_english: "", long_native: "", },
        "aze" => Lang { short: "aze", long_english: "", long_native: "", },
        "bak" => Lang { short: "bak", long_english: "", long_native: "", },
        "bal" => Lang { short: "bal", long_english: "", long_native: "", },
        "bam" => Lang { short: "bam", long_english: "", long_native: "", },
        "ban" => Lang { short: "ban", long_english: "", long_native: "", },
        "bar" => Lang { short: "bar", long_english: "", long_native: "", },
        "bcl" => Lang { short: "bcl", long_english: "", long_native: "", },
        "bel" => Lang { short: "bel", long_english: "", long_native: "", },
        "ben" => Lang { short: "ben", long_english: "", long_native: "", },
        "ber" => Lang { short: "ber", long_english: "", long_native: "", },
        "bfz" => Lang { short: "bfz", long_english: "", long_native: "", },
        "bho" => Lang { short: "bho", long_english: "", long_native: "", },
        "bis" => Lang { short: "bis", long_english: "", long_native: "", },
        "bjn" => Lang { short: "bjn", long_english: "", long_native: "", },
        "bod" => Lang { short: "bod", long_english: "", long_native: "", },
        "bom" => Lang { short: "bom", long_english: "", long_native: "", },
        "bos" => Lang { short: "bos", long_english: "", long_native: "", },
        "bre" => Lang { short: "bre", long_english: "", long_native: "", },
        "brx" => Lang { short: "brx", long_english: "", long_native: "", },
        "bua" => Lang { short: "bua", long_english: "", long_native: "", },
        "bul" => Lang { short: "bul", long_english: "", long_native: "", },
        "bvy" => Lang { short: "bvy", long_english: "", long_native: "", },
        "bzt" => Lang { short: "bzt", long_english: "", long_native: "", },
        "cat" => Lang { short: "cat", long_english: "", long_native: "", },
        "cay" => Lang { short: "cay", long_english: "", long_native: "", },
        "cbk" => Lang { short: "cbk", long_english: "", long_native: "", },
        "ceb" => Lang { short: "ceb", long_english: "", long_native: "", },
        "ces" => Lang { short: "ces", long_english: "", long_native: "", },
        "cha" => Lang { short: "cha", long_english: "", long_native: "", },
        "che" => Lang { short: "che", long_english: "", long_native: "", },
        "chg" => Lang { short: "chg", long_english: "", long_native: "", },
        "chn" => Lang { short: "chn", long_english: "", long_native: "", },
        "cho" => Lang { short: "cho", long_english: "", long_native: "", },
        "chr" => Lang { short: "chr", long_english: "", long_native: "", },
        "chv" => Lang { short: "chv", long_english: "", long_native: "", },
        "cjy" => Lang { short: "cjy", long_english: "", long_native: "", },
        "ckb" => Lang { short: "ckb", long_english: "", long_native: "", },
        "ckt" => Lang { short: "ckt", long_english: "", long_native: "", },
        "cmn" => Lang { short: "cmn", long_english: "", long_native: "", },
        "cmo" => Lang { short: "cmo", long_english: "", long_native: "", },
        "cor" => Lang { short: "cor", long_english: "", long_native: "", },
        "cos" => Lang { short: "cos", long_english: "", long_native: "", },
        "cpi" => Lang { short: "cpi", long_english: "", long_native: "", },
        "crh" => Lang { short: "crh", long_english: "", long_native: "", },
        "crk" => Lang { short: "crk", long_english: "", long_native: "", },
        "crs" => Lang { short: "crs", long_english: "", long_native: "", },
        "csb" => Lang { short: "csb", long_english: "", long_native: "", },
        "cycl" => Lang { short: "cycl", long_english: "", long_native: "", },
        "cym" => Lang { short: "cym", long_english: "", long_native: "", },
        "cyo" => Lang { short: "cyo", long_english: "", long_native: "", },
        "dan" => Lang { short: "dan", long_english: "", long_native: "", },
        "dar" => Lang { short: "dar", long_english: "", long_native: "", },
        "deu" => Lang { short: "deu", long_english: "", long_native: "Deutsch", },
        "diq" => Lang { short: "diq", long_english: "", long_native: "", },
        "div" => Lang { short: "div", long_english: "", long_native: "", },
        "dng" => Lang { short: "dng", long_english: "", long_native: "", },
        "drt" => Lang { short: "drt", long_english: "", long_native: "", },
        "dsb" => Lang { short: "dsb", long_english: "", long_native: "", },
        "dtp" => Lang { short: "dtp", long_english: "", long_native: "", },
        "dws" => Lang { short: "dws", long_english: "", long_native: "", },
        "egl" => Lang { short: "egl", long_english: "", long_native: "", },
        "ell" => Lang { short: "ell", long_english: "", long_native: "", },
        "emx" => Lang { short: "emx", long_english: "", long_native: "", },
        "eng" => Lang { short: "eng", long_english: "", long_native: "English", },
        "enm" => Lang { short: "enm", long_english: "", long_native: "", },
        "epo" => Lang { short: "epo", long_english: "", long_native: "", },
        "est" => Lang { short: "est", long_english: "", long_native: "", },
        "eus" => Lang { short: "eus", long_english: "", long_native: "", },
        "evn" => Lang { short: "evn", long_english: "", long_native: "", },
        "ewe" => Lang { short: "ewe", long_english: "", long_native: "", },
        "ext" => Lang { short: "ext", long_english: "", long_native: "", },
        "fao" => Lang { short: "fao", long_english: "", long_native: "", },
        "fij" => Lang { short: "fij", long_english: "", long_native: "", },
        "fin" => Lang { short: "fin", long_english: "", long_native: "", },
        "fkv" => Lang { short: "fkv", long_english: "", long_native: "", },
        "fra" => Lang { short: "fra", long_english: "", long_native: "", },
        "frm" => Lang { short: "frm", long_english: "", long_native: "", },
        "fro" => Lang { short: "fro", long_english: "", long_native: "", },
        "frr" => Lang { short: "frr", long_english: "", long_native: "", },
        "fry" => Lang { short: "fry", long_english: "", long_native: "", },
        "fuc" => Lang { short: "fuc", long_english: "", long_native: "", },
        "fur" => Lang { short: "fur", long_english: "", long_native: "", },
        "fuv" => Lang { short: "fuv", long_english: "", long_native: "", },
        "gaa" => Lang { short: "gaa", long_english: "", long_native: "", },
        "gag" => Lang { short: "gag", long_english: "", long_native: "", },
        "gan" => Lang { short: "gan", long_english: "", long_native: "", },
        "gbm" => Lang { short: "gbm", long_english: "", long_native: "", },
        "gcf" => Lang { short: "gcf", long_english: "", long_native: "", },
        "gil" => Lang { short: "gil", long_english: "", long_native: "", },
        "gla" => Lang { short: "gla", long_english: "", long_native: "", },
        "gle" => Lang { short: "gle", long_english: "", long_native: "", },
        "glg" => Lang { short: "glg", long_english: "", long_native: "", },
        "glv" => Lang { short: "glv", long_english: "", long_native: "", },
        "gom" => Lang { short: "gom", long_english: "", long_native: "", },
        "gos" => Lang { short: "gos", long_english: "", long_native: "", },
        "got" => Lang { short: "got", long_english: "", long_native: "", },
        "grc" => Lang { short: "grc", long_english: "", long_native: "", },
        "grn" => Lang { short: "grn", long_english: "", long_native: "", },
        "gsw" => Lang { short: "gsw", long_english: "", long_native: "", },
        "guc" => Lang { short: "guc", long_english: "", long_native: "", },
        "guj" => Lang { short: "guj", long_english: "", long_native: "", },
        "hak" => Lang { short: "hak", long_english: "", long_native: "", },
        "hat" => Lang { short: "hat", long_english: "", long_native: "", },
        "hau" => Lang { short: "hau", long_english: "", long_native: "", },
        "haw" => Lang { short: "haw", long_english: "", long_native: "", },
        "hax" => Lang { short: "hax", long_english: "", long_native: "", },
        "hbo" => Lang { short: "hbo", long_english: "", long_native: "", },
        "hdn" => Lang { short: "hdn", long_english: "", long_native: "", },
        "heb" => Lang { short: "heb", long_english: "", long_native: "", },
        "hif" => Lang { short: "hif", long_english: "", long_native: "", },
        "hil" => Lang { short: "hil", long_english: "", long_native: "", },
        "hin" => Lang { short: "hin", long_english: "", long_native: "", },
        "hnj" => Lang { short: "hnj", long_english: "", long_native: "", },
        "hoc" => Lang { short: "hoc", long_english: "", long_native: "", },
        "hrv" => Lang { short: "hrv", long_english: "", long_native: "", },
        "hrx" => Lang { short: "hrx", long_english: "", long_native: "", },
        "hsb" => Lang { short: "hsb", long_english: "", long_native: "", },
        "hsn" => Lang { short: "hsn", long_english: "", long_native: "", },
        "hun" => Lang { short: "hun", long_english: "", long_native: "", },
        "hye" => Lang { short: "hye", long_english: "", long_native: "", },
        "iba" => Lang { short: "iba", long_english: "", long_native: "", },
        "ibo" => Lang { short: "ibo", long_english: "", long_native: "", },
        "ido" => Lang { short: "ido", long_english: "", long_native: "", },
        "iii" => Lang { short: "iii", long_english: "", long_native: "", },
        "ike" => Lang { short: "ike", long_english: "", long_native: "", },
        "ile" => Lang { short: "ile", long_english: "", long_native: "", },
        "ilo" => Lang { short: "ilo", long_english: "", long_native: "", },
        "ina" => Lang { short: "ina", long_english: "", long_native: "", },
        "ind" => Lang { short: "ind", long_english: "", long_native: "", },
        "inh" => Lang { short: "inh", long_english: "", long_native: "", },
        "isl" => Lang { short: "isl", long_english: "", long_native: "", },
        "ita" => Lang { short: "ita", long_english: "", long_native: "", },
        "izh" => Lang { short: "izh", long_english: "", long_native: "", },
        "jam" => Lang { short: "jam", long_english: "", long_native: "", },
        "jav" => Lang { short: "jav", long_english: "", long_native: "", },
        "jbo" => Lang { short: "jbo", long_english: "", long_native: "", },
        "jdt" => Lang { short: "jdt", long_english: "", long_native: "", },
        "jpa" => Lang { short: "jpa", long_english: "", long_native: "", },
        "jpn" => Lang { short: "jpn", long_english: "", long_native: "?????????", },
        "kaa" => Lang { short: "kaa", long_english: "", long_native: "", },
        "kab" => Lang { short: "kab", long_english: "", long_native: "", },
        "kal" => Lang { short: "kal", long_english: "", long_native: "", },
        "kam" => Lang { short: "kam", long_english: "", long_native: "", },
        "kan" => Lang { short: "kan", long_english: "", long_native: "", },
        "kas" => Lang { short: "kas", long_english: "", long_native: "", },
        "kat" => Lang { short: "kat", long_english: "", long_native: "", },
        "kaz" => Lang { short: "kaz", long_english: "", long_native: "", },
        "kbd" => Lang { short: "kbd", long_english: "", long_native: "", },
        "kek" => Lang { short: "kek", long_english: "", long_native: "", },
        "kha" => Lang { short: "kha", long_english: "", long_native: "", },
        "khm" => Lang { short: "khm", long_english: "", long_native: "", },
        "kin" => Lang { short: "kin", long_english: "", long_native: "", },
        "kir" => Lang { short: "kir", long_english: "", long_native: "", },
        "kiu" => Lang { short: "kiu", long_english: "", long_native: "", },
        "kjh" => Lang { short: "kjh", long_english: "", long_native: "", },
        "klj" => Lang { short: "klj", long_english: "", long_native: "", },
        "kmr" => Lang { short: "kmr", long_english: "", long_native: "", },
        "koi" => Lang { short: "koi", long_english: "", long_native: "", },
        "kor" => Lang { short: "kor", long_english: "", long_native: "", },
        "kpv" => Lang { short: "kpv", long_english: "", long_native: "", },
        "krc" => Lang { short: "krc", long_english: "", long_native: "", },
        "krl" => Lang { short: "krl", long_english: "", long_native: "", },
        "ksh" => Lang { short: "ksh", long_english: "", long_native: "", },
        "kum" => Lang { short: "kum", long_english: "", long_native: "", },
        "kxi" => Lang { short: "kxi", long_english: "", long_native: "", },
        "kzj" => Lang { short: "kzj", long_english: "", long_native: "", },
        "laa" => Lang { short: "laa", long_english: "", long_native: "", },
        "lad" => Lang { short: "lad", long_english: "", long_native: "", },
        "lao" => Lang { short: "lao", long_english: "", long_native: "", },
        "lat" => Lang { short: "lat", long_english: "", long_native: "", },
        "lbe" => Lang { short: "lbe", long_english: "", long_native: "", },
        "ldn" => Lang { short: "ldn", long_english: "", long_native: "", },
        "lez" => Lang { short: "lez", long_english: "", long_native: "", },
        "lfn" => Lang { short: "lfn", long_english: "", long_native: "", },
        "lij" => Lang { short: "lij", long_english: "", long_native: "", },
        "lim" => Lang { short: "lim", long_english: "", long_native: "", },
        "lin" => Lang { short: "lin", long_english: "", long_native: "", },
        "lit" => Lang { short: "lit", long_english: "", long_native: "", },
        "liv" => Lang { short: "liv", long_english: "", long_native: "", },
        "lkt" => Lang { short: "lkt", long_english: "", long_native: "", },
        "lld" => Lang { short: "lld", long_english: "", long_native: "", },
        "lmo" => Lang { short: "lmo", long_english: "", long_native: "", },
        "lou" => Lang { short: "lou", long_english: "", long_native: "", },
        "ltg" => Lang { short: "ltg", long_english: "", long_native: "", },
        "ltz" => Lang { short: "ltz", long_english: "", long_native: "", },
        "lug" => Lang { short: "lug", long_english: "", long_native: "", },
        "lut" => Lang { short: "lut", long_english: "", long_native: "", },
        "lvs" => Lang { short: "lvs", long_english: "", long_native: "", },
        "lzh" => Lang { short: "lzh", long_english: "", long_native: "", },
        "lzz" => Lang { short: "lzz", long_english: "", long_native: "", },
        "mad" => Lang { short: "mad", long_english: "", long_native: "", },
        "mah" => Lang { short: "mah", long_english: "", long_native: "", },
        "mai" => Lang { short: "mai", long_english: "", long_native: "", },
        "mal" => Lang { short: "mal", long_english: "", long_native: "", },
        "mar" => Lang { short: "mar", long_english: "", long_native: "", },
        "max" => Lang { short: "max", long_english: "", long_native: "", },
        "mdf" => Lang { short: "mdf", long_english: "", long_native: "", },
        "mfe" => Lang { short: "mfe", long_english: "", long_native: "", },
        "mgm" => Lang { short: "mgm", long_english: "", long_native: "", },
        "mhr" => Lang { short: "mhr", long_english: "", long_native: "", },
        "mic" => Lang { short: "mic", long_english: "", long_native: "", },
        "min" => Lang { short: "min", long_english: "", long_native: "", },
        "mkd" => Lang { short: "mkd", long_english: "", long_native: "", },
        "mlg" => Lang { short: "mlg", long_english: "", long_native: "", },
        "mlt" => Lang { short: "mlt", long_english: "", long_native: "", },
        "mnc" => Lang { short: "mnc", long_english: "", long_native: "", },
        "mni" => Lang { short: "mni", long_english: "", long_native: "", },
        "mnr" => Lang { short: "mnr", long_english: "", long_native: "", },
        "mnw" => Lang { short: "mnw", long_english: "", long_native: "", },
        "moh" => Lang { short: "moh", long_english: "", long_native: "", },
        "mon" => Lang { short: "mon", long_english: "", long_native: "", },
        "mri" => Lang { short: "mri", long_english: "", long_native: "", },
        "mrj" => Lang { short: "mrj", long_english: "", long_native: "", },
        "mus" => Lang { short: "mus", long_english: "", long_native: "", },
        "mvv" => Lang { short: "mvv", long_english: "", long_native: "", },
        "mwl" => Lang { short: "mwl", long_english: "", long_native: "", },
        "mww" => Lang { short: "mww", long_english: "", long_native: "", },
        "mya" => Lang { short: "mya", long_english: "", long_native: "", },
        "myv" => Lang { short: "myv", long_english: "", long_native: "", },
        "nah" => Lang { short: "nah", long_english: "", long_native: "", },
        "nan" => Lang { short: "nan", long_english: "", long_native: "", },
        "nau" => Lang { short: "nau", long_english: "", long_native: "", },
        "nav" => Lang { short: "nav", long_english: "", long_native: "", },
        "nch" => Lang { short: "nch", long_english: "", long_native: "", },
        "nds" => Lang { short: "nds", long_english: "", long_native: "", },
        "new" => Lang { short: "new", long_english: "", long_native: "", },
        "ngt" => Lang { short: "ngt", long_english: "", long_native: "", },
        "ngu" => Lang { short: "ngu", long_english: "", long_native: "", },
        "niu" => Lang { short: "niu", long_english: "", long_native: "", },
        "nld" => Lang { short: "nld", long_english: "", long_native: "", },
        "nlv" => Lang { short: "nlv", long_english: "", long_native: "", },
        "nno" => Lang { short: "nno", long_english: "", long_native: "", },
        "nob" => Lang { short: "nob", long_english: "", long_native: "", },
        "nog" => Lang { short: "nog", long_english: "", long_native: "", },
        "non" => Lang { short: "non", long_english: "", long_native: "", },
        "nov" => Lang { short: "nov", long_english: "", long_native: "", },
        "npi" => Lang { short: "npi", long_english: "", long_native: "", },
        "nst" => Lang { short: "nst", long_english: "", long_native: "", },
        "nus" => Lang { short: "nus", long_english: "", long_native: "", },
        "nya" => Lang { short: "nya", long_english: "", long_native: "", },
        "nys" => Lang { short: "nys", long_english: "", long_native: "", },
        "oar" => Lang { short: "oar", long_english: "", long_native: "", },
        "oci" => Lang { short: "oci", long_english: "", long_native: "", },
        "ofs" => Lang { short: "ofs", long_english: "", long_native: "", },
        "oji" => Lang { short: "oji", long_english: "", long_native: "", },
        "ood" => Lang { short: "ood", long_english: "", long_native: "", },
        "ori" => Lang { short: "ori", long_english: "", long_native: "", },
        "orv" => Lang { short: "orv", long_english: "", long_native: "", },
        "osp" => Lang { short: "osp", long_english: "", long_native: "", },
        "oss" => Lang { short: "oss", long_english: "", long_native: "", },
        "osx" => Lang { short: "osx", long_english: "", long_native: "", },
        "ota" => Lang { short: "ota", long_english: "", long_native: "", },
        "otk" => Lang { short: "otk", long_english: "", long_native: "", },
        "pag" => Lang { short: "pag", long_english: "", long_native: "", },
        "pal" => Lang { short: "pal", long_english: "", long_native: "", },
        "pam" => Lang { short: "pam", long_english: "", long_native: "", },
        "pan" => Lang { short: "pan", long_english: "", long_native: "", },
        "pap" => Lang { short: "pap", long_english: "", long_native: "", },
        "pau" => Lang { short: "pau", long_english: "", long_native: "", },
        "pcd" => Lang { short: "pcd", long_english: "", long_native: "", },
        "pdc" => Lang { short: "pdc", long_english: "", long_native: "", },
        "pes" => Lang { short: "pes", long_english: "", long_native: "", },
        "pfl" => Lang { short: "pfl", long_english: "", long_native: "", },
        "phn" => Lang { short: "phn", long_english: "", long_native: "", },
        "pli" => Lang { short: "pli", long_english: "", long_native: "", },
        "pms" => Lang { short: "pms", long_english: "", long_native: "", },
        "pnb" => Lang { short: "pnb", long_english: "", long_native: "", },
        "pol" => Lang { short: "pol", long_english: "", long_native: "", },
        "por" => Lang { short: "por", long_english: "", long_native: "", },
        "ppl" => Lang { short: "ppl", long_english: "", long_native: "", },
        "prg" => Lang { short: "prg", long_english: "", long_native: "", },
        "pus" => Lang { short: "pus", long_english: "", long_native: "", },
        "quc" => Lang { short: "quc", long_english: "", long_native: "", },
        "que" => Lang { short: "que", long_english: "", long_native: "", },
        "qxq" => Lang { short: "qxq", long_english: "", long_native: "", },
        "qya" => Lang { short: "qya", long_english: "", long_native: "", },
        "rap" => Lang { short: "rap", long_english: "", long_native: "", },
        "rel" => Lang { short: "rel", long_english: "", long_native: "", },
        "rif" => Lang { short: "rif", long_english: "", long_native: "", },
        "roh" => Lang { short: "roh", long_english: "", long_native: "", },
        "rom" => Lang { short: "rom", long_english: "", long_native: "", },
        "ron" => Lang { short: "ron", long_english: "", long_native: "", },
        "rue" => Lang { short: "rue", long_english: "", long_native: "", },
        "run" => Lang { short: "run", long_english: "", long_native: "", },
        "rus" => Lang { short: "rus", long_english: "", long_native: "", },
        "ryu" => Lang { short: "ryu", long_english: "", long_native: "", },
        "sag" => Lang { short: "sag", long_english: "", long_native: "", },
        "sah" => Lang { short: "sah", long_english: "", long_native: "", },
        "san" => Lang { short: "san", long_english: "", long_native: "", },
        "sat" => Lang { short: "sat", long_english: "", long_native: "", },
        "scn" => Lang { short: "scn", long_english: "", long_native: "", },
        "sco" => Lang { short: "sco", long_english: "", long_native: "", },
        "sdh" => Lang { short: "sdh", long_english: "", long_native: "", },
        "sgs" => Lang { short: "sgs", long_english: "", long_native: "", },
        "shi" => Lang { short: "shi", long_english: "", long_native: "", },
        "shs" => Lang { short: "shs", long_english: "", long_native: "", },
        "shy" => Lang { short: "shy", long_english: "", long_native: "", },
        "sin" => Lang { short: "sin", long_english: "", long_native: "", },
        "sjn" => Lang { short: "sjn", long_english: "", long_native: "", },
        "slk" => Lang { short: "slk", long_english: "", long_native: "", },
        "slv" => Lang { short: "slv", long_english: "", long_native: "", },
        "sma" => Lang { short: "sma", long_english: "", long_native: "", },
        "sme" => Lang { short: "sme", long_english: "", long_native: "", },
        "smo" => Lang { short: "smo", long_english: "", long_native: "", },
        "sna" => Lang { short: "sna", long_english: "", long_native: "", },
        "snd" => Lang { short: "snd", long_english: "", long_native: "", },
        "som" => Lang { short: "som", long_english: "", long_native: "", },
        "sot" => Lang { short: "sot", long_english: "", long_native: "", },
        "spa" => Lang { short: "spa", long_english: "", long_native: "Espa??ol", },
        "sqi" => Lang { short: "sqi", long_english: "", long_native: "", },
        "srd" => Lang { short: "srd", long_english: "", long_native: "", },
        "srn" => Lang { short: "srn", long_english: "", long_native: "", },
        "srp" => Lang { short: "srp", long_english: "", long_native: "", },
        "ssw" => Lang { short: "ssw", long_english: "", long_native: "", },
        "stq" => Lang { short: "stq", long_english: "", long_native: "", },
        "sun" => Lang { short: "sun", long_english: "", long_native: "", },
        "sux" => Lang { short: "sux", long_english: "", long_native: "", },
        "swe" => Lang { short: "swe", long_english: "", long_native: "", },
        "swg" => Lang { short: "swg", long_english: "", long_native: "", },
        "swh" => Lang { short: "swh", long_english: "", long_native: "", },
        "syc" => Lang { short: "syc", long_english: "", long_native: "", },
        "tah" => Lang { short: "tah", long_english: "", long_native: "", },
        "tam" => Lang { short: "tam", long_english: "", long_native: "", },
        "tat" => Lang { short: "tat", long_english: "", long_native: "", },
        "tel" => Lang { short: "tel", long_english: "", long_native: "", },
        "tet" => Lang { short: "tet", long_english: "", long_native: "", },
        "tgk" => Lang { short: "tgk", long_english: "", long_native: "", },
        "tgl" => Lang { short: "tgl", long_english: "", long_native: "", },
        "tha" => Lang { short: "tha", long_english: "", long_native: "", },
        "thv" => Lang { short: "thv", long_english: "", long_native: "", },
        "tig" => Lang { short: "tig", long_english: "", long_native: "", },
        "tir" => Lang { short: "tir", long_english: "", long_native: "", },
        "tkl" => Lang { short: "tkl", long_english: "", long_native: "", },
        "tlh" => Lang { short: "tlh", long_english: "", long_native: "", },
        "tly" => Lang { short: "tly", long_english: "", long_native: "", },
        "tmr" => Lang { short: "tmr", long_english: "", long_native: "", },
        "tmw" => Lang { short: "tmw", long_english: "", long_native: "", },
        "toi" => Lang { short: "toi", long_english: "", long_native: "", },
        "toki" => Lang { short: "toki", long_english: "", long_native: "", },
        "ton" => Lang { short: "ton", long_english: "", long_native: "", },
        "tpi" => Lang { short: "tpi", long_english: "", long_native: "", },
        "tpw" => Lang { short: "tpw", long_english: "", long_native: "", },
        "tsn" => Lang { short: "tsn", long_english: "", long_native: "", },
        "tso" => Lang { short: "tso", long_english: "", long_native: "", },
        "tts" => Lang { short: "tts", long_english: "", long_native: "", },
        "tuk" => Lang { short: "tuk", long_english: "", long_native: "", },
        "tur" => Lang { short: "tur", long_english: "", long_native: "", },
        "tvl" => Lang { short: "tvl", long_english: "", long_native: "", },
        "tyv" => Lang { short: "tyv", long_english: "", long_native: "", },
        "tzl" => Lang { short: "tzl", long_english: "", long_native: "", },
        "udm" => Lang { short: "udm", long_english: "", long_native: "", },
        "uig" => Lang { short: "uig", long_english: "", long_native: "", },
        "ukr" => Lang { short: "ukr", long_english: "", long_native: "", },
        "umb" => Lang { short: "umb", long_english: "", long_native: "", },
        "unknown" => Lang { short: "unknown", long_english: "", long_native: "Unknown", },
        "urd" => Lang { short: "urd", long_english: "", long_native: "", },
        "urh" => Lang { short: "urh", long_english: "", long_native: "", },
        "uzb" => Lang { short: "uzb", long_english: "", long_native: "", },
        "vec" => Lang { short: "vec", long_english: "", long_native: "", },
        "vep" => Lang { short: "vep", long_english: "", long_native: "", },
        "vie" => Lang { short: "vie", long_english: "", long_native: "", },
        "vol" => Lang { short: "vol", long_english: "", long_native: "", },
        "vro" => Lang { short: "vro", long_english: "", long_native: "", },
        "war" => Lang { short: "war", long_english: "", long_native: "", },
        "wln" => Lang { short: "wln", long_english: "", long_native: "", },
        "wol" => Lang { short: "wol", long_english: "", long_native: "", },
        "wuu" => Lang { short: "wuu", long_english: "", long_native: "", },
        "xal" => Lang { short: "xal", long_english: "", long_native: "", },
        "xho" => Lang { short: "xho", long_english: "", long_native: "", },
        "xmf" => Lang { short: "xmf", long_english: "", long_native: "", },
        "xqa" => Lang { short: "xqa", long_english: "", long_native: "", },
        "yid" => Lang { short: "yid", long_english: "", long_native: "", },
        "yor" => Lang { short: "yor", long_english: "", long_native: "", },
        "yua" => Lang { short: "yua", long_english: "", long_native: "", },
        "yue" => Lang { short: "yue", long_english: "", long_native: "", },
        "zea" => Lang { short: "zea", long_english: "", long_native: "", },
        "zgh" => Lang { short: "zgh", long_english: "", long_native: "", },
        "zlm" => Lang { short: "zlm", long_english: "", long_native: "", },
        "zsm" => Lang { short: "zsm", long_english: "", long_native: "", },
        "zul" => Lang { short: "zul", long_english: "", long_native: "", },
        "zza" => Lang { short: "zza", long_english: "", long_native: "", },
    };
}
