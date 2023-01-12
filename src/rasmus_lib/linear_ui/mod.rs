use std::cmp::Ordering;

use crate::ansi_term_old::Command;
use crate::ansi_term_old::CursorMovement::*;
use crate::utils::str_count_lines;
use crate::utils::StrUtils;
use crate::*;

pub mod implementations;
pub mod widgets;

pub trait Clearable {
    fn line_count(&self) -> u32;

    fn clear_lines(&self) {
        if self.line_count() > 0 {
            ansi_term_old::perform_commands(&[Command::EraseUp(self.line_count())]);
        }
        // else if self.line_count() == 1 {
        //     ansi_term::perform_commands(&[Command::EraseLine]);
        // }
    }
}

pub trait Renderable {
    fn render(&self) -> Buffer;
}

pub trait Frameable {
    fn frame(self, frame: Frame) -> Self;
}

#[derive(Debug)]
pub struct Render {
    line_count: u32,
    content: String,
}

impl Render {
    pub fn from_buffer(buffer: Buffer) -> Self {
        Self {
            line_count: buffer.content.chars().filter(|char| *char == '\n').count() as u32,
            content: buffer.content,
        }
    }
    pub fn from_string(string: String) -> Self {
        Self {
            line_count: string.chars().filter(|char| *char == '\n').count() as u32,
            content: string,
        }
    }

    pub fn replace_content(&mut self, buffer: Buffer) {
        self.line_count = str_count_lines(&buffer.content) as u32;
        self.content = buffer.content;
    }

    pub fn rerender(&self) {
        print!("{}", self.content);
    }
}

impl Clearable for Render {
    fn line_count(&self) -> u32 {
        self.line_count
    }
}

#[derive(Debug)]
pub struct Buffer {
    content: String,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn from_string(string: String) -> Self {
        Self { content: string }
    }

    pub fn push(&mut self, string: &str) {
        self.content.push_str(string);
    }
    pub fn pushln(&mut self, string: &str) {
        self.content.push_str(string);
        self.content.push_str("\r\n");
    }

    pub fn push_char(&mut self, char: char) {
        self.content.push(char);
    }

    pub fn push_buffer(&mut self, buffer: Buffer) {
        self.content.push_str(&buffer.content);
    }

    pub fn push_render(&mut self, render: Render) {
        self.content.push_str(&render.content);
    }

    pub fn line_count(&self) -> u32 {
        self.content.chars().filter(|char| char == &'\n').count() as u32
    }

    pub fn render(self) -> Render {
        print!("{}", self.content);
        Render::from_string(self.content)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Frameable for Buffer {
    fn frame(self, frame: Frame) -> Self {
        let content = self.content;
        // let height = content.lines().count();

        // print_crlf!("Framing buffer");

        let width = content
            .lines()
            .map(|line| line.visual_len())
            .max()
            .unwrap_or(0);
        let frame_width = width + frame.left.visual_len() + frame.right.visual_len() - 2;

        let mut out = Buffer::new();
        out.push_char(frame.top_left_corner);
        out.push(&frame.top.extend_to_length(frame_width));
        out.push_char(frame.top_right_corner);
        out.push("\r\n");

        for line in content.lines() {
            // print_crlf!("Handling line {}", line);
            out.push(&frame.left);

            let content_width = line.grapheme_len();
            let extra_width = width - content_width;
            out.push(line);
            if extra_width != 0 {
                out.push(&" ".repeat(extra_width));
            }

            out.push(&frame.right);
            out.push("\r\n");
        }

        out.push_char(frame.bottom_left_corner);

        out.push(&frame.bottom.extend_to_length(frame_width));

        out.push_char(frame.bottom_right_corner);
        out.push("\r\n");

        out
    }
}

pub struct Frame {
    top: String,
    bottom: String,
    left: String,
    right: String,
    top_left_corner: char,
    top_right_corner: char,
    bottom_left_corner: char,
    bottom_right_corner: char,
}

impl Frame {
    pub fn new(
        top: &str,
        bottom: &str,
        left: &str,
        right: &str,
        corners: (char, char, char, char),
    ) -> Self {
        Self {
            top: top.to_owned(),
            bottom: bottom.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
            top_left_corner: corners.0,
            top_right_corner: corners.1,
            bottom_left_corner: corners.2,
            bottom_right_corner: corners.3,
        }
    }
}

#[derive(Debug)]
pub struct Movement {
    lines_up: i32,
}

impl Movement {
    pub fn create_and_move(lines_up: i32) -> Self {
        match lines_up.cmp(&0) {
            Ordering::Greater => {
                ansi_term_old::perform_commands(&[Command::Move(Up(lines_up as u32))]);
            }
            Ordering::Less => {
                ansi_term_old::perform_commands(&[Command::Move(Down((-lines_up) as u32))]);
            }
            Ordering::Equal => {}
        }

        Self { lines_up }
    }

    pub fn append_and_move(&mut self, lines_up: i32) {
        match lines_up.cmp(&0) {
            Ordering::Greater => {
                ansi_term_old::perform_commands(&[Command::Move(Up(lines_up as u32))]);
            }
            Ordering::Less => {
                ansi_term_old::perform_commands(&[Command::Move(Down((-lines_up) as u32))]);
            }
            Ordering::Equal => {}
        }

        self.lines_up += lines_up;
    }

    pub fn undo(self) {
        match self.lines_up.cmp(&0) {
            Ordering::Greater => {
                ansi_term_old::perform_commands(&[Command::Move(Up(self.lines_up as u32))]);
            }
            Ordering::Less => {
                ansi_term_old::perform_commands(&[Command::Move(Down((-self.lines_up) as u32))]);
            }
            Ordering::Equal => {}
        }
    }
}

pub struct UiScene {
    renders: Vec<(usize, Render)>,
    current_id: usize,
}

impl UiScene {
    pub const fn new() -> Self {
        Self {
            renders: Vec::new(),
            current_id: 0,
        }
    }

    pub fn append(&mut self, buffer: Buffer) -> usize {
        // let allocated = Box::new(render);
        self.renders.push((self.current_id, buffer.render()));
        self.current_id += 1;

        self.current_id - 1
        // self.renders.last().unwrap().as_ref()
    }

    pub fn insert(&mut self, render_id: usize, buffer: Buffer) -> Option<usize> {
        if let Some(index) = self.get_render_index(render_id) {
            for (_, render) in self.renders[index..].iter().rev() {
                // println!("cleared {} moving up {}", i, render.line_count);

                render.clear_lines();

                // movement.append_and_move(render.line_count as i32);
            }

            self.renders
                .insert(index, (self.current_id, buffer.render()));
            self.current_id += 1;

            if index == self.renders.len() - 1 {
                return Some(self.current_id - 1);
            }

            // movement.append_and_move()

            for (_, render) in self.renders[(index + 1)..].iter() {
                // println!("Rerendered {}", i);
                render.rerender();
            }

            // dbg!(movement);

            return Some(self.current_id - 1);
        }
        None
    }

    pub fn include_render(&mut self, render: Render) -> usize {
        // print_crlf!("included render {}", self.current_id);
        self.renders.push((self.current_id, render));
        self.current_id += 1;

        self.current_id - 1
    }

    pub fn remove_render(&mut self, render_id: usize) -> Option<()> {
        if let Some(index) = self.get_render_index(render_id) {
            // let (_, render) = &self.renders[index];

            // let height: i32 = self.renders[index..]
            //     .iter()
            //     .map(|(_, render)| render.line_count)
            //     .sum::<u32>() as i32;

            // movement.append_and_move(4);
            // movement.append_and_move(4);
            // ansi_term::perform_commands(&[Command::EraseUp(4)]);
            // print!("hi \n");
            self.clear_up_to(index);

            if index == self.renders.len() - 1 {
                self.renders.remove(index);
                return Some(());
            }

            // movement.append_and_move()

            for (_, render) in self.renders[(index + 1)..].iter() {
                // print_crlf!("Rerendered {}", i);
                render.rerender();
            }

            // dbg!(movement);

            self.renders.remove(index);
            return Some(());
        }
        None
    }

    pub fn replace_render(&mut self, render_id: usize, buffer: Buffer) -> Option<()> {
        if let Some(index) = self.get_render_index(render_id) {
            self.clear_up_to(index);

            let (_, render) = &mut self.renders[index];

            render.replace_content(buffer);
            // render.rerender();

            // if index == self.renders.len() - 1 {
            //     return Some(());
            // }

            // dbg!(&self.renders.len());

            for (_, render) in self.renders[index..].iter() {
                // println!("Rerendered {}", i);
                render.rerender();
            }

            return Some(());
        }
        None
    }

    fn clear_up_to(&self, index: usize) {
        for (_, render) in self.renders[index..].iter().rev() {
            // println!("cleared {} moving up {}", i, render.line_count);
            render.clear_lines();
        }
    }

    fn get_render_index(&self, render_id: usize) -> Option<usize> {
        // dbg!(&self.renders);
        self.renders.iter().position(|item| item.0 == render_id)
    }
}
