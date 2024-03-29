use std::io;

use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    input::{args::Args, config::Config},
    instance::{selection::Selection, InstanceRunable},
    state::{handler::StateHandler, runtime_state::RuntimeState},
};

pub fn run(config: Config, args: Args) -> Result<(), &'static str> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = match Terminal::new(backend) {
        Ok(terminal) => terminal,
        Err(_) => return Err("Failed to create a TUI terminal"),
    };

    let volume = config.starting_volume.clamp(0, 100);
    let speed = config.starting_speed.clamp(10, 200);
    let state = RuntimeState::new(volume, speed);

    let paths = match args.load {
        Some(audio) => audio,
        None => match Config::get_audio_directory_content(config.audio_directory.as_str()) {
            Ok(paths) => paths,
            Err(err) => return Err(err),
        },
    };

    let mut selection = Selection::new(paths);
    let mut handler = StateHandler::new(state, config, terminal);
    selection.run(&mut handler);

    Ok(())
}
