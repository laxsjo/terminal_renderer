use crossterm::event;
use crossterm::event::{Event, KeyEvent};
// use crossterm::event::{KeyCode, KeyModifiers};
// use lazy_static::lazy_static;
// use std::cell::RefCell;
use static_init::dynamic;
use std::io::Read;
use std::io::Write;
use std::io::{self, stdout};
use std::str::FromStr;
use std::sync::mpsc::{self, Sender};
use std::sync::Mutex;
use std::time::Duration;
use std::{str, thread};

use self::string_editor::StringEditor;
use crate::ansi_term_old::Command;
use crate::linear_ui::{Buffer, Clearable, Render};
use crate::utils::PeekableReceiver;
use crate::utils::StrUtils;
use crate::{ansi_term_old, print_crlf};

pub mod string_editor;

pub use crossterm::event::KeyCode;
pub use crossterm::event::KeyModifiers;

#[dynamic]
pub static INPUT: Input = Input::new();
// lazy_static! {
// }

pub enum InputError {
    IoError(io::Error),
    ParseError,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct InputEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl InputEvent {
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

impl From<KeyEvent> for InputEvent {
    fn from(event: KeyEvent) -> Self {
        Self {
            code: event.code,
            modifiers: event.modifiers,
        }
    }
}

pub struct Input {
    receiver: Mutex<PeekableReceiver<InputEvent>>,
}

impl Input {
    fn handle_event(event: InputEvent, sender: &Sender<InputEvent>) {
        match event {
            InputEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                crate::clean_up();
                // TerminalFlags::clean_up();
                // flush_commands();
                // crossterm::terminal::disable_raw_mode().expect("Help oh god why");

                print_crlf!("Terminated!");

                std::process::exit(130);
            }
            event => {
                // print_crlf!("Sent code {:?}", event.code);
                sender.send(event).expect("Sending failed :(");
            }
        }
    }

    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || loop {
            // if !event::poll(Duration::from_millis(500)).unwrap() {
            //     continue;
            // }

            if let Event::Key(event) = event::read().unwrap() {
                // print_crlf!("from thread: {:?}", event.code);
                Self::handle_event(event.into(), &sender);
            }
        });

        Self {
            receiver: Mutex::new(PeekableReceiver::new(receiver)),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.receiver
            .lock()
            .expect("couldn't lock mutex")
            .is_empty()
            .expect("input channel disconnected")
    }

    /// Returns an iterator over input events.
    /// The iterator yields all currently available events, returning `None`
    /// when no more are.
    pub fn iter(&self) -> InputIter<'_> {
        InputIter { input: self }
    }

    /// Returns an iterator over input events, blocking until events become
    /// available.
    ///
    /// The iterator *only* yields `None` if the input channel has hanged up.
    pub fn iter_blocking(&self) -> InputBlockingIter<'_> {
        InputBlockingIter { input: self }
    }

    /// Attempts to waits for the next input event, returning `None` if the input
    /// channel has hanged up.
    ///
    /// # Panics
    ///
    /// Panics if the input sender thread has panicked, causing the mutex lock
    /// to fail.
    pub fn get_event(&self) -> Option<InputEvent> {
        let event = self
            .receiver
            .lock()
            .expect("couldn't lock input mutex")
            .recv();

        event.ok()
    }

    /// Returns the next available input event if available, returning `None` if
    /// all inputs have been processed or the input channel has disconnected
    /// (maybe it should panic if that's the case?).
    ///
    /// # Panics
    ///
    /// Panics if the input sender thread has panicked, causing the mutex lock
    /// to fail.
    pub fn try_get_event(&self) -> Option<InputEvent> {
        let event = self
            .receiver
            .lock()
            .expect("couldn't lock input mutex")
            .try_recv();

        event.ok()
    }

    pub fn read_char(&self) -> char {
        loop {
            let event = self
                .receiver
                .lock()
                .expect("Couldn't unlock mutex")
                .recv()
                .expect("Input failed");
            match event {
                InputEvent {
                    code: KeyCode::Char(char),
                    ..
                } => return char,
                InputEvent {
                    code: KeyCode::Tab, ..
                } => return '\t',
                InputEvent {
                    code: KeyCode::Enter,
                    ..
                } => return '\n',
                InputEvent {
                    code: KeyCode::Backspace,
                    ..
                } => return '\x7F',
                _ => {}
            }
        }
    }

    pub fn loop_input<F: FnMut(InputEvent) -> bool>(&self, mut f: F) {
        loop {
            let event = self
                .receiver
                .lock()
                .expect("Couldn't unlock mutex")
                .recv()
                .expect("Input failed");

            if !f(event) {
                break;
            }
        }
    }

    pub fn wait_for_key(&self, key: KeyCode) {
        self.loop_input(|event| event.code != key)
    }

    /// Reads string, ending when user presses enter
    ///
    /// # Panics
    ///
    /// Panics if stdin couldn't be read or stdin.flush fails.
    pub fn read_string(&self) -> String {
        let mut editor = StringEditor::new();

        // let mut stdout = io::stdout();

        // let mut error = None;
        self.loop_input(|key| {
            let input: Option<String> = match key {
                InputEvent {
                    code: KeyCode::Char(char),
                    ..
                } => Some(char.to_string()),
                InputEvent {
                    code: KeyCode::Enter,
                    ..
                } => Some("\r\n".to_string()),
                InputEvent {
                    code: KeyCode::Tab, ..
                } => Some('\t'.to_string()),
                InputEvent {
                    code: KeyCode::Backspace,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    editor.delete();
                    None
                }
                InputEvent {
                    code: KeyCode::Backspace,
                    modifiers: KeyModifiers::ALT,
                    ..
                } => {
                    editor.delete_word();
                    None
                }
                InputEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    editor.move_left();
                    None
                }
                InputEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    editor.move_left_word();
                    None
                }
                InputEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    editor.move_right();
                    None
                }
                InputEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    editor.move_right_word();
                    None
                }
                _ => None,
            };
            if let Some(input) = input {
                editor.write(&input);
                if input == "\r\n" {
                    return false;
                }
            }
            true
        });

        print!("\r\n");

        editor.close() + "\r\n"
    }

    fn read_input<T: FromStr>(&self) -> InputOut<T> {
        io::stdout().flush().expect("couldn't flush stdout");

        let input = self.read_string();

        InputOut {
            result: match input.trim().parse::<T>() {
                Ok(value) => Ok(value),
                Err(_) => Err(InputError::ParseError),
            },
            render: Render::from_string(input),
        }
    }

    pub fn ask_input<T: FromStr>(&self, prompt: &str) -> InputValue<T> {
        // println!("{}", prompt);
        let prompt_lines = prompt.chars().filter(|char| char == &'\n').count() as u32;

        loop {
            print!("{}", prompt);

            let input = self.read_input::<T>();
            match input.result {
                Ok(value) => {
                    let mut buffer = Buffer::from_string(prompt.to_string());
                    buffer.push_render(input.render);
                    return InputValue {
                        value,
                        // input_lines: input.input_lines + prompt_lines,
                        render: Render::from_buffer(buffer),
                    };
                }
                Err(e) => match e {
                    InputError::IoError(_) => panic!("Couldn't read stdin"),
                    InputError::ParseError => {
                        ansi_term_old::perform_commands(&[Command::EraseUp(
                            input.render.line_count() + prompt_lines,
                        )]);

                        continue;
                    }
                },
            };
        }
    }

    pub fn ask_input_filter<T, F>(&self, prompt: &str, validator: F) -> InputValue<T>
    where
        T: FromStr,
        F: Fn(&T) -> bool,
    {
        // println!("{}", prompt);

        let prompt_lines = prompt.chars().filter(|char| char == &'\n').count() as u32;

        loop {
            print!("{}", prompt);

            let input = self.read_input::<T>();
            match input.result.and_then(|value| {
                if validator(&value) {
                    Ok(value)
                } else {
                    Err(InputError::ParseError)
                }
            }) {
                Ok(value) => {
                    let mut buffer = Buffer::from_string(prompt.to_string());
                    buffer.push_render(input.render);
                    return InputValue {
                        value,
                        // input_lines: input.input_lines + prompt_lines,
                        render: Render::from_buffer(buffer),
                    };
                }
                Err(e) => match e {
                    InputError::IoError(_) => panic!("Couldn't read stdin"),
                    InputError::ParseError => {
                        ansi_term_old::perform_commands(&[Command::EraseUp(
                            input.render.line_count() + prompt_lines,
                        )]);

                        continue;
                    }
                },
            };
        }
    }

    pub fn ask_input_map<T, U, F>(&self, prompt: &str, mapper: F) -> InputValue<U>
    where
        T: FromStr,
        F: Fn(&T) -> Option<U>,
    {
        let prompt_lines = prompt.chars().filter(|char| char == &'\n').count() as u32;

        loop {
            print!("{}", prompt);

            let input = self.read_input::<T>();
            match input.result.and_then(|value| match mapper(&value) {
                Some(value) => Ok(value),
                None => Err(InputError::ParseError),
            }) {
                Ok(value) => {
                    let mut buffer = Buffer::from_string(prompt.to_string());
                    buffer.push_render(input.render);
                    return InputValue {
                        value,
                        // input_lines: input.input_lines + prompt_lines,
                        render: Render::from_buffer(buffer),
                    };
                }
                Err(e) => match e {
                    InputError::IoError(_) => panic!("Couldn't read stdin"),
                    InputError::ParseError => {
                        ansi_term_old::perform_commands(&[Command::EraseUp(
                            input.render.line_count() + prompt_lines,
                        )]);

                        continue;
                    }
                },
            };
        }
    }

    pub fn ask_bool(&self, prompt: &str) -> InputValue<bool> {
        let prompt_line_count = prompt.line_count();
        let mut stdout = stdout();
        loop {
            print!("{}", prompt);
            stdout.flush().expect("Couldn't flush stdout");

            let char = self.read_char();

            if char != 'y' && char != 'n' {
                ansi_term_old::perform_commands(&[Command::EraseUp(prompt_line_count as u32)]);
                print!("\r");
                continue;
            }

            let mut buffer = Buffer::from_string(prompt.to_string());
            buffer.push_char(char);

            print!("\r\n");
            stdout.flush().expect("Couldn't flush stdout");
            buffer.push("\r\n");

            return InputValue {
                value: char == 'y',
                render: Render::from_buffer(buffer),
            };
        }
    }
}

pub struct InputIter<'a> {
    input: &'a Input,
}

impl<'a> Iterator for InputIter<'a> {
    type Item = InputEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.try_get_event()
    }
}

pub struct InputBlockingIter<'a> {
    input: &'a Input,
}

impl<'a> Iterator for InputBlockingIter<'a> {
    type Item = InputEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.get_event()
    }
}

pub struct InputOut<T: FromStr> {
    pub result: Result<T, InputError>,
    // input_lines: u32,
    pub render: Render,
}

// impl<T: FromStr> Clearable for InputResult<T> {
//     fn line_count(&self) -> u32 {
//         self.render.line_count()
//     }
// }

pub struct InputValue<T> {
    pub value: T,
    // input_lines: u32,
    pub render: Render,
}

pub type InputResult<T> = std::result::Result<T, InputError>;

// pub enum InputResult<T> {
//     Ok(T),
//     Err(InputError),
// }

// impl<T> From<crossterm::Result<T>> for InputResult<T> {
//     fn from(result: crossterm::Result<T>) -> Self {
//         return match result {
//             Ok(value) => InputResult::Ok(value),
//             Err(err) => InputResult::Err(InputError::IoError(err)),
//         };
//     }
// }

#[deprecated]
pub fn read_char() -> crossterm::Result<char> {
    loop {
        if !event::poll(Duration::from_millis(500))? {
            continue;
        }

        if let Event::Key(event) = event::read()? {
            match event {
                KeyEvent {
                    code: KeyCode::Char(char),
                    ..
                } => return Ok(char),
                KeyEvent {
                    code: KeyCode::Tab, ..
                } => return Ok('\t'),
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => return Ok('\n'),
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => return Ok('\x7F'),
                _ => {}
            }
        }
    }
}

/// Calls function over raw input bytes.
///
/// Probably not very useful, use loop_input() instead
///
/// # Panics
///
/// Panics if stdin().read() fails
#[deprecated]
#[allow(deprecated)]
pub fn loop_input_bytes<F: Fn(u8)>(f: F, break_byte: u8) {
    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf[0] != break_byte {
        f(buf[0]);
    }
}

#[deprecated]
#[allow(deprecated)]
pub fn loop_input<F: FnMut(KeyEvent) -> bool>(mut f: F) -> crossterm::Result<()> {
    loop {
        if !event::poll(Duration::from_millis(500))? {
            continue;
        }

        if let Event::Key(event) = event::read()? {
            if !f(event) {
                break;
            }
        }
    }

    Ok(())
}

// impl<T: FromStr> Clearable for InputValue<T> {
//     fn line_count(&self) -> u32 {
//         self.input_lines
//     }
// }

#[deprecated]
#[allow(deprecated)]
pub fn wait_for_key(key: KeyCode) -> crossterm::Result<()> {
    loop_input(|event| event.code != key)
}

/// Reads string, ending when user presses enter
///
/// # Panics
///
/// Panics if stdin couldn't be read or stdin.flush fails.
#[deprecated]
#[allow(deprecated)]
pub fn read_string() -> InputResult<String> {
    let mut editor = StringEditor::new();

    // let mut stdout = io::stdout();

    // let mut error = None;
    if let Err(err) = loop_input(|key| {
        let input: Option<String> = match key {
            KeyEvent {
                code: KeyCode::Char(char),
                ..
            } => Some(char.to_string()),
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => Some("\r\n".to_string()),
            KeyEvent {
                code: KeyCode::Tab, ..
            } => Some('\t'.to_string()),
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                editor.delete();
                None
            }
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::ALT,
                ..
            } => {
                editor.delete_word();
                None
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                editor.move_left();
                None
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                editor.move_left_word();
                None
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                editor.move_right();
                None
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                editor.move_right_word();
                None
            }
            _ => None,
        };
        if let Some(input) = input {
            editor.write(&input);
            if input == "\r\n" {
                return false;
            }
        }
        true
    }) {
        return Err(InputError::IoError(err));
    };

    // if let Some(error) = error {
    //     return Err(error);
    // }

    // let mut bytes: Vec<u8> = Vec::new();
    // let mut out = String::new();

    // let mut char_buffer = [0u8; 4];
    // let mut current_index = 0;
    // let mut buf = [0; 1];
    // while io::stdin().read(&mut buf).expect("Failed to read line") == 1 {
    //     char_buffer[current_index] = buf[0];
    //     current_index += 1;
    //     if let Ok(string) = str::from_utf8(&char_buffer[0..current_index]) {
    //         let char = string.chars().next().expect("invalid utf-8 byte sequence");
    //         out.push(char);
    //         print!("{}", char);
    //         if char == '\r' {
    //             out.push('\n');
    //             print!("\n");
    //         }
    //         current_index = 0;

    //         io::stdout().flush().expect("Couldn't flush output");

    //         if char == '\r' {
    //             break;
    //         }
    //     }
    //     // println!("{}\r", buf[0]);
    //     // print!("{}", buf[0] as char);
    //     // io::stdout().flush().expect("Couldn't flush output");
    //     // let char = buf[0] as char;
    //     bytes.push(buf[0]);
    // }
    print!("\r\n");

    // // let out_bytes = String::from_utf8(bytes).expect("Invalid utf-8 input");

    Ok(editor.close() + "\r\n")
}

#[deprecated]
#[allow(deprecated)]
fn read_input<T: FromStr>() -> InputOut<T> {
    io::stdout().flush().expect("couldn't flush stdout");

    let input;
    match read_string() {
        Ok(string) => input = string,
        Err(err) => {
            return InputOut {
                result: Err(err),
                render: Render::from_string("".to_owned()),
            }
        }
    }
    // if let Err(_) = io::stdin().read_line(&mut input) {
    //     return InputResult {
    //         result: Err(InputError::IoError),
    //         // input_lines: 0,
    //         render: Render::from_string(input),
    //     };
    // }

    // // let input_lines = input.chars().filter(|char| char == &'\n').count() as u32;

    // // let input = input.trim();
    InputOut {
        result: match input.trim().parse::<T>() {
            Ok(value) => Ok(value),
            Err(_) => Err(InputError::ParseError),
        },
        render: Render::from_string(input),
    }
}

#[deprecated]
#[allow(deprecated)]
pub fn ask_input<T: FromStr>(prompt: &str) -> InputValue<T> {
    // println!("{}", prompt);
    let prompt_lines = prompt.chars().filter(|char| char == &'\n').count() as u32;

    loop {
        print!("{}", prompt);

        let input = read_input::<T>();
        match input.result {
            Ok(value) => {
                let mut buffer = Buffer::from_string(prompt.to_string());
                buffer.push_render(input.render);
                return InputValue {
                    value,
                    // input_lines: input.input_lines + prompt_lines,
                    render: Render::from_buffer(buffer),
                };
            }
            Err(e) => match e {
                InputError::IoError(_) => panic!("Couldn't read stdin"),
                InputError::ParseError => {
                    ansi_term_old::perform_commands(&[Command::EraseUp(
                        input.render.line_count() + prompt_lines,
                    )]);

                    continue;
                }
            },
        };
    }
}

#[deprecated]
#[allow(deprecated)]
pub fn ask_input_filter<T, F>(prompt: &str, validator: F) -> InputValue<T>
where
    T: FromStr,
    F: Fn(&T) -> bool,
{
    // println!("{}", prompt);

    let prompt_lines = prompt.chars().filter(|char| char == &'\n').count() as u32;

    loop {
        print!("{}", prompt);

        let input = read_input::<T>();
        match input.result.and_then(|value| {
            if validator(&value) {
                Ok(value)
            } else {
                Err(InputError::ParseError)
            }
        }) {
            Ok(value) => {
                let mut buffer = Buffer::from_string(prompt.to_string());
                buffer.push_render(input.render);
                return InputValue {
                    value,
                    // input_lines: input.input_lines + prompt_lines,
                    render: Render::from_buffer(buffer),
                };
            }
            Err(e) => match e {
                InputError::IoError(_) => panic!("Couldn't read stdin"),
                InputError::ParseError => {
                    ansi_term_old::perform_commands(&[Command::EraseUp(
                        input.render.line_count() + prompt_lines,
                    )]);

                    continue;
                }
            },
        };
    }
}

#[deprecated]
#[allow(deprecated)]
pub fn ask_input_map<T, U, F>(prompt: &str, mapper: F) -> InputValue<U>
where
    T: FromStr,
    F: Fn(&T) -> Option<U>,
{
    let prompt_lines = prompt.chars().filter(|char| char == &'\n').count() as u32;

    loop {
        print!("{}", prompt);

        let input = read_input::<T>();
        match input.result.and_then(|value| match mapper(&value) {
            Some(value) => Ok(value),
            None => Err(InputError::ParseError),
        }) {
            Ok(value) => {
                let mut buffer = Buffer::from_string(prompt.to_string());
                buffer.push_render(input.render);
                return InputValue {
                    value,
                    // input_lines: input.input_lines + prompt_lines,
                    render: Render::from_buffer(buffer),
                };
            }
            Err(e) => match e {
                InputError::IoError(_) => panic!("Couldn't read stdin"),
                InputError::ParseError => {
                    ansi_term_old::perform_commands(&[Command::EraseUp(
                        input.render.line_count() + prompt_lines,
                    )]);

                    continue;
                }
            },
        };
    }
}

#[deprecated]
#[allow(deprecated)]
pub fn ask_bool(prompt: &str) -> InputValue<bool> {
    let prompt_line_count = prompt.line_count();
    let mut stdout = stdout();
    loop {
        print!("{}", prompt);
        stdout.flush().expect("Couldn't flush stdout");

        let Ok(char) = read_char() else {
            panic!("Couldn't read stdin");
        };
        if char != 'y' && char != 'n' {
            ansi_term_old::perform_commands(&[Command::EraseUp(prompt_line_count as u32)]);
            print!("\r");
            continue;
        }

        let mut buffer = Buffer::from_string(prompt.to_string());
        buffer.push_char(char);

        print!("\r\n");
        stdout.flush().expect("Couldn't flush stdout");
        buffer.push("\r\n");

        return InputValue {
            value: char == 'y',
            render: Render::from_buffer(buffer),
        };
    }
}
