//! User-facing configuration.

/// The configuration to be used when running the Ferru "build" subcommand.
#[derive(Clone, Debug)]
pub struct Config<'a> {
    /// The directory to be used as the source for the website being generated.
    pub source_directory: Option<&'a str>,
    /// The directory to be used as the destination for the website being
    /// generated.
    pub dest_directory: Option<&'a str>,
}
