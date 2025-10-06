use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::env;
use crate::command_executor;

pub struct ShellState {
    pub input: String,
    pub output: Vec<String>,
    pub current_dir: String,
    pub confirm_exit:bool,
    pub show_output: bool,
    pub dark_mode: bool,
    command_sender: Sender<String>,
    output_receiver: Receiver<String>,

}

impl ShellState {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx) = channel::<String>();
        let (out_tx, out_rx) = channel::<String>();

        thread::spawn(move || {
            while let Ok(command) = cmd_rx.recv() {
                let result = command_executor::execute_command(&command);
                let _ = out_tx.send(result);
            }
        });

        Self {
            input: String::new(),
            output: vec!["".to_string()],
            current_dir: env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "/".to_string()),
            confirm_exit: false,
            show_output: true,
            dark_mode: false,
            command_sender: cmd_tx,
            output_receiver: out_rx,
        }
    }

    pub fn update_from_receiver(&mut self) {
        while let Ok(output) = self.output_receiver.try_recv() {
            self.output.push(output);
            self.update_current_dir();
        }
    }

    pub fn update_current_dir(&mut self) {
        self.current_dir = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "/".to_string());
    }

    pub fn execute_command(&mut self, command: String) {
        self.output.push(format!("Xi9d> {}", command));
        let _ = self.command_sender.send(command);
    }

    pub fn clear_output(&mut self) {
        self.output.clear();
        self.output.push("Xi9d Shell".to_string());
    }
}

