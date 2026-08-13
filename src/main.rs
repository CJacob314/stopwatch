#![warn(clippy::all)]

use humantime::format_duration;
use iced::{
    Element,
    Length::Fill,
    Subscription, time,
    widget::{button, container, text},
};
use std::time::{Duration, SystemTime};

#[derive(Default)]
struct AppState {
    running: bool,
    start_time: Option<SystemTime>,
    last_time_string: String,
}

impl AppState {
    fn new() -> Self {
        Default::default()
    }
}

fn main() -> iced::Result {
    iced::application(AppState::new, update, view)
        .subscription(subscription)
        .title("Stopwatch")
        .antialiasing(true)
        .window(iced::window::Settings {
            size: (300, 150).into(),
            closeable: true,
            minimizable: true,
            decorations: true,
            blur: true,
            transparent: true,
            ..Default::default()
        })
        .centered()
        .run()
}

fn update(state: &mut AppState, message: Message) {
    match message {
        Message::StartStop => {
            state.running = !state.running;
            if state.running {
                state.start_time = Some(SystemTime::now());
            } else {
                // Assign last_time_string so we can continue showing the last time in the view
                state.last_time_string = state
                    .start_time
                    .unwrap()
                    .elapsed()
                    .map(|dur| format_duration(dur).to_string())
                    .unwrap_or_else(|_| "ERROR".to_owned());
            }
        }
        Message::Update => {}
    }
}

fn subscription(_: &AppState) -> Subscription<Message> {
    // 16ms ~= 60 times a second
    time::every(Duration::from_millis(16)).map(|_| Message::Update)
}

fn view(state: &AppState) -> Element<'_, Message> {
    let txt = if state.running {
        match state.start_time.as_ref() {
            Some(start_time) => start_time
                .elapsed()
                .map(|dur| format_duration(dur).to_string())
                .unwrap_or_else(|_| "ERROR".to_owned()),
            None => "Start/Stop".to_owned(),
        }
    } else {
        if state.last_time_string.is_empty() {
            "Start/Stop".to_owned()
        } else {
            state.last_time_string.clone()
        }
    };

    container(button(text(txt)).on_press(Message::StartStop))
        .center_x(Fill)
        .center_y(Fill)
        .into()
}

#[derive(Debug, Clone)]
enum Message {
    /// Stopwatch start/stop command
    StartStop,
    /// To update the UI. Produced by an `iced::time::every` [Subscription]
    ///
    /// [Subscription]: iced::Subscription
    Update,
}
