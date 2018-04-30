extern crate termion;
extern crate tui;

use std::io;
use std::process::Command;
use std::process::Output;
use std::time;
use std::thread;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, SelectableList, Widget};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Modifier, Style};

fn init() -> Terminal<MouseBackend> {
    let backend = MouseBackend::new().unwrap();
    Terminal::new(backend).unwrap()
}

fn download(url: &str) -> io::Result<Output> {
    Command::new("youtube-dl")
        .arg("--extract-audio")
        .args(&["--audio-format", "mp3"])
        .args(&["--output", "./dl/%(title)s.%(ext)s"])
        .arg(url)
        .output()
}

struct App<'a> {
    items: Vec<&'a str>,
    selected: usize,
    size: Rect,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: vec!["foo", "bar"],
            selected: 0,
            size: Rect::default(),
        }
    }
}

enum Event {
    Input(event::Key),
    Tick,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let clock_tx = tx.clone();
    let mut terminal = init();
    let mut app = App::new();

    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });
    
    /*
    let handle = thread::spawn(move || -> Output {
        download("https://www.youtube.com/watch?v=GYqfVnH187g").unwrap()
    });

    println!("waiting on join");
    let result = handle.join();
    match result {
        Ok(output) => {
            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(err) => {
            println!("got err: {:?}", err);
        }
    }
    */

    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    app.size = terminal.size().unwrap();
    draw(&mut terminal, &app);

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('q') => {
                    break;
                }

                _ => {}
            }

            Event::Tick => {}
        }

        draw(&mut terminal, &app);
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(100)])
        .margin(1)
        .render(t, &app.size, |t, chunks| {
            let style = Style::default().fg(Color::Black).bg(Color::White);
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("List"))
                .items(&app.items)
                .highlight_symbol("+")
                .select(1)
                .render(t, &chunks[0]);
        });

    t.draw().unwrap();
}
