use iced::widget::{button, center, column};
use iced::window;
use iced::{Center, Element, Task};
use rodio::{OutputStream, Sink, Decoder};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::sync::{Arc, Mutex};

pub fn main() -> iced::Result {
    iced::application("AfchisApp - Iced", MyApp::update, MyApp::view)
        .run_with(|| (MyApp::new(), iced::Task::none()))
}

#[derive(Debug, Clone, Copy)]
enum Message {
    PlayAudio,
    Confirm,
    Exit,
}

#[derive(Default)]
struct MyApp {
    sink: Option<Arc<Mutex<Sink>>>, // Хранение Sink
    show_confirm: bool,
}

impl MyApp {
    fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap(); // Используем unwrap для обработки ошибки
        println!("Initilate MyApp");

        Self {
            show_confirm: false,
            sink: Some(Arc::new(Mutex::new(sink))), // Инициализация Sink
        } 
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PlayAudio => {
                if let Some(sink) = &self.sink {
                    let sink_clone = Arc::clone(sink); // Клонируем Arc
                    thread::spawn(move || Self::play_audio(sink_clone));
                }
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

    fn play_audio(sink: Arc<Mutex<Sink>>) {
        // Загрузка аудиофайла
        let file = BufReader::new(File::open("./.data/sound.mp3").unwrap());
        let source = Decoder::new(file).unwrap();
        
        let sink = sink.lock().unwrap();

        // Воспроизведение звука
        sink.append(source);
        println!("Audio source appended to sink.");
        sink.sleep_until_end(); // Блокировка потока до завершения воспроизведения
    }
}
