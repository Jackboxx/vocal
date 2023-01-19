use std::io;

use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    audio::init::init_audio_handler,
    input::{args::Args, config::Config},
    instance::{audio_instance::AudioInstance, selection_instace::SelectionInstance},
    properties::runtime_properties::RuntimeOptions,
};

pub fn run(config: Config, args: Args) -> Result<(), &'static str> {
    let (mut sink, _stream) = match init_audio_handler() {
        Some(handler_data) => handler_data,
        None => return Err("Failed to create audio sink"),
    };

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(terminal) => terminal,
        Err(_) => return Err("Failed to create a TUI terminal"),
    };

    let mut runtime_options = RuntimeOptions::new(50, 100);
    sink.set_speed(runtime_options.speed_decimal);
    sink.set_volume(runtime_options.volume_decimal);

    match args.play {
        Some(paths) => {
            for path in paths {
                AudioInstance::start_instance(
                    path,
                    &mut sink,
                    &mut runtime_options,
                    &config,
                    &mut terminal,
                )
            }
        }
        None => {
            let paths = match args.load {
                Some(audio) => audio,
                None => {
                    match Config::get_audio_directory_content(config.audio_directory.as_str()) {
                        Ok(paths) => paths,
                        Err(err) => return Err(err),
                    }
                }
            };

            let mut selection_instance = SelectionInstance::new(paths);

            match selection_instance.show_selection(
                &mut sink,
                &mut runtime_options,
                &config,
                &mut terminal,
            ) {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }
    };
    Ok(())
}
