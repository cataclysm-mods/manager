use clap::{App, Arg, SubCommand, crate_authors, crate_version};
use crate::NAME;

/// When called, parses ARGV 
///
/// Example:
/// ```
/// parse_arguments(["cataclysm-manager", "--version"])
/// assert!(
///     error.description,
///     "Unable to parse release data due to semantically invalid JSON data when attempting to extract Greeting at 1:12:"
/// )
/// ```
pub fn parse_arguments(argv: Vec<String>) -> clap::ArgMatches<'static> {
    let app = App::new(NAME)
        .author(crate_authors!())
        .version(crate_version!())

        .arg(
            Arg::with_name("log-level")
                .help("Log level")
                .long("log-level")
                .default_value("info")
                .possible_values(&["error", "warn", "info", "debug", "trace"]))

        .subcommand(
            SubCommand::with_name("releases")
                .about("List or download available game releases.")

                .subcommand(
                    SubCommand::with_name("list")
                        .about("List available game releases.")
                        .arg(
                            Arg::with_name("RELEASE_TYPE")
                                .required(true)
                                .help("Type of release to download.")
                                .possible_values(&["experimental", "stable"]))));

    app.get_matches_from(argv)
}
