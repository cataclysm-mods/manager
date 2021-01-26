use clap::{App, Arg, SubCommand, crate_authors, crate_version};
use crate::NAME;

/// When called, parses ARGV using Clap
///
/// Example:
/// ```
/// use cataclysm_manager::cli::parse_arguments;
/// let args = vec!["cataclysm-manager", "--version"];
/// let argv = args.iter().map(|s| s.to_string()).collect();
/// let matches = parse_arguments(argv);
/// assert_eq!(matches.is_present("version"), true);
/// assert_eq!(matches.occurrences_of("version"), 1);
/// assert_eq!(matches.occurrences_of("help"), 0);
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
