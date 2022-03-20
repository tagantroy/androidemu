use log::*;
use simplelog::*;
use std::env;
use std::error::Error;

fn init_logger() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // WriteLogger::new(
        //     LevelFilter::Info,
        //     Config::default(),
        //     File::create("~/.androidemu/main.log").unwrap(),
        // ),
    ])
    .unwrap();
}

fn show_available_emulators() -> Result<(), Box<dyn Error>> {
    let emulators = get_existing_emulators();

    let items: Vec<powerpack::Item> = emulators
        .into_iter()
        .map(|name| {
            powerpack::Item::new(name.clone())
                .icon(powerpack::Icon::with_type("public.script"))
                .arg(name)
        })
        .collect();
    powerpack::output(items)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    init_logger();
    let query = env::args().nth(1);
    match query {
        None => show_available_emulators(),
        Some(_) => {
            let emulator_name = env::args().nth(2).expect("Emulator name expected");
            start_emulator(emulator_name)
        },
    }
}

fn get_emulator_bin_path() -> String {
    powerpack::env::var("emulator_binary_path").expect("Emulator path should be set.")
}

fn start_emulator(name: String) -> Result<(), Box<dyn Error>> {
    let emulator_path = get_emulator_bin_path();
    std::process::Command::new(emulator_path)
        .arg(format!("@{}", name))
        .output()
        .expect("Cannot start emulator");
    Ok(())
}

fn get_existing_emulators() -> Vec<String> {
    let emulator_path = get_emulator_bin_path();
    let res = std::process::Command::new(emulator_path)
        .arg("-list-avds")
        .output()
        .expect("Cannot start emulator process");
    let emulators =
        std::string::String::from_utf8(res.stdout).expect("Cannot extract emulator list");
    emulators.lines().map(|v| v.to_string()).collect()
}
