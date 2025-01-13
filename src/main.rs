use iced::widget::{button, center, column};
use iced::window;
use iced::{Center, Element, Task};
use rodio::{OutputStream, Sink, Decoder};
use std::io::BufReader;
use std::fs::File;
use std::thread;

pub fn main() -> iced::Result {
    iced::application("MyApp - Iced", MyApp::update, MyApp::view).run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    PlayAudio,
    Confirm,
    Exit,
}

#[derive(Default)]
struct MyApp {
    show_confirm: bool,
    is_playing: bool,
}

impl MyApp {

    fn new() -> Self {
        Self {
            show_confirm: false,
            is_playing: false,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PlayAudio => {
                thread::spawn(|| Self::play_audio());
                Task::none()
            }
            Message::Confirm => window::get_latest().and_then(window::close),
            Message::Exit => {
                self.show_confirm = true;

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = if self.show_confirm {
            column![
                "Are you sure you want to exit?",
                button("Yes, exit now")
                    .padding([10, 20])
                    .on_press(Message::Confirm),
            ]
        } else {
            column![
                "Play audio or quit",
                button("Play audio").padding([10, 20]).on_press(Message::PlayAudio),
                button("Exit").padding([10, 20]).on_press(Message::Exit),
            ]
        }
        .spacing(10)
        .align_x(Center);

        center(content).padding(20).into()
    }

    fn play_audio() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Load an audio file
        let file = BufReader::new(File::open("./.data/afchis_track_1.mp3").unwrap());
        let source = Decoder::new(file).unwrap();

        // Play the sound
        sink.append(source);
        sink.sleep_until_end();
    }
}

