use std::{
    thread,
    time::{Duration, Instant},
};

use crossterm::event::KeyCode;
use rodio::{OutputStream, Sink, Source};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
};

use crate::{
    audio::{init::init_audio_handler, source_data::SourceData},
    events::{event::trigger, player_events::PlayerEvent, queue_events::QueueEvent},
    input::{
        key::{poll_key, Key},
        player_keybindings::{get_player_keybindings, process_player_input},
    },
    instance::{queue::Queue, Instance, InstanceRunableWithParent},
    render::{
        bar::draw_bar,
        chart::{create_data_from_samples, draw_chart},
        footer::draw_footer,
        info::draw_info,
    },
    state::{audio_state::AudioState, handler::StateHandler},
};

pub struct Player {
    pub sink: Sink,
    // ====================================================================================================
    // this field has to be stored because if it goes out of scope no audio
    // will play through the sink
    // =================================================================================================
    _stream: OutputStream,
    pub source_data: SourceData,
    pub state: AudioState,
}

impl InstanceRunableWithParent<Queue> for Player {
    fn run<B: Backend>(&mut self, handler: &mut StateHandler<B>, parent: &mut Queue) {
        trigger(PlayerEvent::Start, handler, self);
        let terminal_size = match handler.get_terminal_size() {
            Ok(size) => size,
            Err(err) => {
                log::error!("-ERROR- Failed to get terminal size: {err}");
                return;
            }
        };

        let source = match SourceData::get_source(&self.source_data.path) {
            Ok(source) => source,
            Err(err) => {
                log::error!("-ERROR- Failed to audio source data: {err}");
                return;
            }
        };

        let interval = 16;
        let sample_rate = source.sample_rate();
        let step = (sample_rate * interval) as f32 / 1000.0;

        self.sink.append(source);
        loop {
            self.tick(handler.get_state().get_speed_decimal().into());

            if parent.interupted || parent.audio_changed {
                trigger(PlayerEvent::Stop, handler, self);
                return;
            }

            let progress = self.state.progress;
            if progress > 1.0 {
                trigger(PlayerEvent::Stop, handler, self);
                trigger(QueueEvent::AudioFinished, handler, parent);
                return;
            }

            let start = (progress * self.source_data.samples.len() as f64) as usize;
            let bar_count = (terminal_size.width / 2) as usize;

            let volume = handler.get_state().volume;
            let speed = handler.get_state().speed;
            let is_muted = handler.get_state().is_muted;
            let show_hotkeys = handler.get_config().show_hotkeys;
            let custom_footer = handler.get_config().custom_footer.clone();
            let bar_width = handler.get_config().bar_width;
            let bar_gap = handler.get_config().bar_gap;
            let color = handler.get_config().get_color();
            let highlight_color = handler.get_config().get_highlight_color();

            match handler.terminal.draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(20),
                            Constraint::Percentage(5),
                            Constraint::Percentage(10),
                            Constraint::Percentage(15),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                let top_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(5),
                            Constraint::Percentage(90),
                            Constraint::Percentage(5),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[0]);

                let max = 10000;
                let multiplier = 100_f32;

                if let Some(data) = create_data_from_samples(
                    self.source_data.samples.clone(),
                    start,
                    step as usize,
                    bar_count,
                    max,
                    multiplier,
                ) {
                    rect.render_widget(
                        draw_chart(
                            data.as_slice(),
                            bar_width,
                            bar_gap,
                            max * multiplier as u64,
                            color,
                        ),
                        top_chunks[1],
                    );
                }

                rect.render_widget(
                    draw_info(
                        &self.source_data.path,
                        volume,
                        is_muted,
                        speed,
                        self.state.duration.as_secs_f64(),
                        self.state.passed_time,
                        highlight_color,
                    ),
                    chunks[1],
                );
                rect.render_widget(draw_bar(progress, color), chunks[3]);

                let text: String = if show_hotkeys {
                    [Player::get_keybindings(), Queue::get_keybindings()]
                        .concat()
                        .iter()
                        .map(|item| format!("  {}", item.hint))
                        .collect()
                } else {
                    custom_footer.unwrap_or("".to_owned())
                };

                rect.render_widget(
                    draw_footer(text, show_hotkeys, color, highlight_color),
                    chunks[4],
                );
            }) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to render frame: {}", err);
                }
            }

            loop {
                if let Some(code) = poll_key() {
                    parent.process_input(handler, code);
                    self.process_input(handler, code);
                }

                if !self.state.is_paused {
                    break;
                } else {
                    self.reset_tick();
                }
            }
            thread::sleep(Duration::from_millis(interval.into()));
        }
    }
}

impl Instance for Player {
    fn get_keybindings() -> Vec<Key> {
        get_player_keybindings()
    }

    fn process_input<B: Backend>(&mut self, handler: &mut StateHandler<B>, code: KeyCode) {
        process_player_input(handler, self, code)
    }
}

impl Player {
    pub fn new(path: &str, volume: f32, speed: f32) -> Option<Player> {
        let source_data = match SourceData::new(path) {
            Some(source_data) => source_data,
            None => return None,
        };

        let duration = source_data.duration;

        let (sink, _stream) = match init_audio_handler() {
            Some(handler_data) => handler_data,
            None => return None,
        };

        sink.set_volume(volume);
        sink.set_speed(speed);

        Some(Player {
            sink,
            _stream,
            source_data,
            state: AudioState::new(duration),
        })
    }

    fn tick(&mut self, speed: f64) {
        self.state.passed_time += self.state.time_since_last_tick.elapsed().as_secs_f64() * speed;
        self.state.time_since_last_tick = Instant::now();

        self.state.progress = self.state.passed_time / self.state.duration.as_secs_f64();
    }

    fn reset_tick(&mut self) {
        self.state.time_since_last_tick = Instant::now();
    }
}
