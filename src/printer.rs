use std::sync::mpsc::{Receiver, channel, Sender};
use std::thread;
use crate::errors::JobError;

enum PrinterMessage {
    Shutdown,
    JobError(JobError),
    Error(Box<str>),
    Line(Box<str>),
    Lines(Vec<Box<str>>),
}

use crate::printer::PrinterMessage::*;

#[derive(Clone)]
pub struct Printer {
    sender: Sender<PrinterMessage>,
}

impl Printer {

    pub fn new() -> Printer {
        let (sender, receiver) = channel();
        thread::Builder::new().name("printer".to_string()).spawn(move || {
            loop {
                match receiver.recv() {
                    Ok(message) => {
                        match message {
                            Shutdown => break,
                            Error(err) => println!("Error: {}", err),
                            JobError(err) => println!("Error: {}", err.message),
                            Line(line) => println!("{}", line),
                            Lines(lines) => for line in lines {println!("{}", line)},
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        Printer {
            sender,
        }
    }

    pub fn shutdown(&self) {
        self.sender.send(PrinterMessage::Shutdown);
    }

    pub fn line(&self, line: &str) {
        self.sender.send(PrinterMessage::Line(Box::from(line)));
    }

    pub fn lines(&self, lines: Vec<Box<str>>) {
        self.sender.send(PrinterMessage::Lines(lines));
    }

    pub fn job_error(&self, err: JobError) {
        self.sender.send(PrinterMessage::JobError(err));
    }

    pub fn error(&self, err: &str) {
        self.sender.send(PrinterMessage::Error(Box::from(err)));
    }
}
