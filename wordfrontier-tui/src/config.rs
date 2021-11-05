/// This is the user-facing configuration for the program.
#[derive(Debug, argh::FromArgs)]
pub struct Config {
    #[argh(option, short = 't')]
    /// short name of the target language, i.e. the language that is to be learned.
    pub target_lang_short_name: String,
    #[argh(option, default = "\"eng\".to_string()", short = 'r')]
    /// short name of the reference language, i.e. the language that translations will be provided in.
    pub reference_lang_short_name: String,
}
