use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

use crate::ansi_term_old;
// use crate::ansi_term::Color;
use crate::ansi_term_old::Command;
use crate::ansi_term_old::CursorMovement::*;
// use crate::ansi_term::Format;
use crate::utils::StrUtils;

pub struct StringEditor {
    pub string: String,
    cursor_position: usize,
    string_length: usize,
}

impl StringEditor {
    pub fn new() -> Self {
        stdout()
            .queue(cursor::Show)
            .expect("I hate this")
            .queue(cursor::EnableBlinking)
            .expect("I hate this")
            .flush()
            .expect("I hate this");
        Self {
            string: String::new(),
            cursor_position: 0,
            string_length: 0,
        }
    }

    pub fn write(&mut self, str: &str) {
        // if str.contains("\n") {
        //     panic!("Oh god! StringEditor doesn't support newline charchters");
        // }
        let filter_string: String;
        let filtered = if str.contains('\n') {
            filter_string = str
                .chars()
                .filter(|char| char != &'\n' && char != &'\r')
                .collect::<String>();
            &filter_string
        } else {
            str
        };

        let length = filtered.chars().count();

        if self.cursor_position >= self.string_length {
            self.string.push_str(filtered);
        } else {
            self.string.insert_str(self.cursor_position, filtered);
        }
        print!(
            "{}",
            self.string
                .chars()
                .skip(self.cursor_position)
                .collect::<String>()
        );

        self.cursor_position += length;
        self.string_length += length;

        ansi_term_old::perform_commands(&[Command::Move(Left(
            (self.string_length - self.cursor_position) as u32,
        ))]);

        stdout()
            .queue(cursor::MoveLeft(1))
            .expect("Hate")
            .queue(cursor::MoveRight(1))
            .expect("Hate")
            .flush()
            .expect("Hate");
        stdout().flush().expect("Help! Flush didn't work");
    }

    pub fn move_left(&mut self) {
        self.move_left_chars(1);
    }
    pub fn move_right(&mut self) {
        self.move_right_chars(1);
    }

    pub fn move_left_word(&mut self) {
        let boundary = self.string.next_word_boundary_left(self.cursor_position);
        self.move_left_chars(self.cursor_position - boundary);
    }
    pub fn move_right_word(&mut self) {
        let boundary = self.string.next_word_boundary_right(self.cursor_position);
        self.move_right_chars(boundary - self.cursor_position);
    }

    pub fn delete(&mut self) {
        if self.string_length == 0 {
            return;
        }
        if self.cursor_position >= self.string_length {
            self.string.pop();

            ansi_term_old::perform_commands(&[Command::Move(Left(1)), Command::EraseToLineEnd]);
            self.cursor_position -= 1;
        } else {
            self.cursor_position -= 1;

            self.string.remove(self.cursor_position);
            ansi_term_old::perform_commands(&[Command::Move(Left(1))]);

            let shifted_string = self
                .string
                .chars()
                .skip(self.cursor_position)
                .collect::<String>();

            print!("{}", shifted_string);
            ansi_term_old::perform_commands(&[Command::Move(Left(shifted_string.len() as u32))]);
        }
        stdout().flush().expect("Help! Flush didn't work");

        self.string_length -= 1;
    }

    pub fn delete_word(&mut self) {
        if self.string_length == 0 {
            return;
        }

        let boundary = self.string.next_word_boundary_left(self.cursor_position);
        // print!("Found boundary {}", boundary);
        // stdout().flush().expect("hi");
        // panic!("hi");
        let length = self.cursor_position - boundary;

        // dbg!(self.cursor_position >= self.string_length)
        // panic!();

        if self.cursor_position >= self.string_length {
            // print!("{}", ansi_term::format(&[Format::Fg(Color::Green)]));
            for _ in 0..length {
                self.string.pop();
            }

            ansi_term_old::perform_commands(&[
                Command::Move(Left(length as u32)),
                Command::EraseToLineEnd,
            ]);
            // print!("erased to end");
            self.cursor_position -= length;
        } else {
            // print!("{}", ansi_term::format(&[Format::Fg(Color::Red)]));
            self.string
                .replace_range(boundary..self.cursor_position, "");
            ansi_term_old::perform_commands(&[Command::Move(Left(length as u32))]);
            self.cursor_position -= length;

            let shifted_string = self
                .string
                .chars()
                .skip(self.cursor_position)
                .collect::<String>();

            print!("{}", shifted_string);
            ansi_term_old::perform_commands(&[Command::Move(Left(shifted_string.len() as u32))]);
        }
        stdout().flush().expect("Help! Flush didn't work");

        self.string_length -= length;
    }

    pub fn close(self) -> String {
        self.string.clone()
    }

    fn move_left_chars(&mut self, count: usize) {
        if count > self.cursor_position {
            return;
        }
        self.cursor_position -= count;
        ansi_term_old::perform_commands(&[Command::Move(Left(count as u32))]);
    }
    fn move_right_chars(&mut self, mut count: usize) {
        if count + self.cursor_position > self.string_length {
            count = self.string_length - self.cursor_position;
        }

        self.cursor_position += count;
        ansi_term_old::perform_commands(&[Command::Move(Right(count as u32))]);
    }
}

impl Default for StringEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StringEditor {
    fn drop(&mut self) {
        stdout()
            .queue(cursor::Hide)
            .expect("I hate this")
            .queue(cursor::DisableBlinking)
            .expect("I hate this")
            .flush()
            .expect("I hate this");
    }
}
