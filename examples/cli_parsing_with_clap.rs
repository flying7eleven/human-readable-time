use clap::Parser;
use human_readable_time::HumanReadableDuration;

#[derive(Parser)]
#[clap(name = "cli_example")]
pub struct SettingsObject {
    ///
    #[clap(long)]
    pub duration: Option<HumanReadableDuration>,
}

/// Call the program by passing a duration with the corresponding flag:
///
/// - cargo run --example cli_parsing_with_clap -- --duration=10s
/// - cargo run --example cli_parsing_with_clap -- --duration=8h5m10s
/// - cargo run --example cli_parsing_with_clap -- --duration=120m
fn main() {
    // get the command line parameters from the user
    let cmd_parameters: SettingsObject = SettingsObject::parse();
    use human_readable_time::traits::{AsHours, AsMinutes, AsSeconds};

    // if there was no duration supplied, terminate early
    if cmd_parameters.duration.is_none() {
        println!("No duration was supplied");
        return;
    }

    // print out the supplied duration
    let duration = cmd_parameters.duration.unwrap();
    println!("Full hours: {}", duration.as_hours());
    println!("Full minutes: {}", duration.as_minutes());
    println!("Full seconds: {}", duration.as_seconds());
}
