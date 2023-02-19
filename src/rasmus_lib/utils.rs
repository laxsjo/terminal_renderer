use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::swap;
use std::str::SplitWhitespace;
use std::sync::mpsc;
use std::time::{Duration, Instant};

// use std::collections::HashSet;
use ansi_parser::{AnsiParser, Output};
use num_traits::*;
use unicode_segmentation::UnicodeSegmentation;

pub trait StrUtils {
    fn grapheme_len(&self) -> usize;
    fn line_count(&self) -> usize;

    /// Returns the displayed length of `self` in a terminal supporting ansi escape sequences.
    ///
    /// This counts the visual characters while ignoring ansi escape sequences.
    fn visual_len(&self) -> usize;

    /// Returns an iterator over the words of a string slice.
    ///
    /// Words are slices seperated by unicode whitespace.
    fn words(&self) -> Words;

    fn next_word_boundary_left(&self, start_position: usize) -> usize;
    fn next_word_boundary_right(&self, start_position: usize) -> usize;

    fn surround_with(&self, before: &str, after: &str) -> String;
    fn extend_to_length(&self, length: usize) -> String;
}

impl StrUtils for str {
    fn grapheme_len(&self) -> usize {
        let is_extended = true;
        self.graphemes(is_extended).count()
    }
    fn line_count(&self) -> usize {
        self.chars().filter(|char| *char == '\n').count()
    }

    fn visual_len(&self) -> usize {
        self.ansi_parse()
            .filter_map(|block| match block {
                Output::Escape(_) => None,
                Output::TextBlock(string) => Some(string),
            })
            .map(|string| {
                string.graphemes(true).count()
                    - string.chars().filter(|char| char.is_control()).count()
            })
            .sum()
    }

    fn words(&self) -> Words {
        Words::new(self)
    }

    fn next_word_boundary_left(&self, mut start_position: usize) -> usize {
        if start_position > self.len() {
            start_position = self.len();
        } else if start_position == 0 {
            return 0;
        }

        let search_str = &self[..start_position];

        let mut searching_for_word = None;

        let mut position = start_position - 1;
        for char in search_str.chars().rev() {
            if char.is_alphanumeric() || char == '_' {
                match searching_for_word {
                    None => searching_for_word = Some(false),
                    Some(true) => return position + 1,
                    _ => {}
                }
            } else {
                match searching_for_word {
                    None => searching_for_word = Some(true),
                    Some(false) => return position + 1,
                    _ => {}
                }
            }

            position = position.saturating_sub(1);
        }

        0
    }
    fn next_word_boundary_right(&self, start_position: usize) -> usize {
        if start_position >= self.len() {
            return self.len();
        }

        let search_str = &self[start_position..];

        let mut searching_for_word = None;

        for (position, char) in search_str.chars().enumerate() {
            if char.is_alphanumeric() || char == '_' {
                match searching_for_word {
                    None => searching_for_word = Some(false),
                    Some(true) => return start_position + position,
                    _ => {}
                }
            } else {
                match searching_for_word {
                    None => searching_for_word = Some(true),
                    Some(false) => return start_position + position,
                    _ => {}
                }
            }
        }

        self.len()
    }
    fn surround_with(&self, before: &str, after: &str) -> String {
        format!("{}{}{}", before, self, after)
    }

    fn extend_to_length(&self, length: usize) -> String {
        // print_crlf!("extending {}", self);
        let part_length = self.grapheme_len();
        let mut out: String = self.repeat(length / part_length);

        if length % part_length != 0 {
            out.push_str(
                &self
                    .graphemes(true)
                    .take(length % part_length)
                    .collect::<String>(),
            )
        }

        out
    }
}

pub struct Words<'a>(SplitWhitespace<'a>);

impl<'a> Words<'a> {
    pub fn new(string: &'a str) -> Self {
        Self(string.split_whitespace())
    }
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub trait VecUtils<T>
where
    T: PartialEq,
{
    fn run_length_encoding(&self) -> Vec<(&T, usize)>;
    fn into_run_length_encoding(self) -> Vec<(T, usize)>;
}

impl<T> VecUtils<T> for Vec<T>
where
    T: PartialEq,
{
    /// Encode vector content using run length encoding, using references to
    /// the original vector.
    ///
    /// Run length encoding is a compression algorithm where each item is
    /// stored as a value and length, such that sequences of identical values
    /// are only stored as a single item.
    ///
    /// Source: https://en.wikipedia.org/wiki/Run-length_encoding
    fn run_length_encoding(&self) -> Vec<(&T, usize)> {
        let mut encoded = Vec::new();

        let mut last_item = None;
        let mut current_length = 0;
        for item in self {
            let Some(current_item) = last_item else {
                last_item = Some(item);
                current_length = 1;
                continue;
            };

            if current_item != item {
                encoded.push((current_item, current_length));
                last_item = Some(item);
                current_length = 0;
            }

            current_length += 1;
        }

        if let Some(current_item) = last_item {
            encoded.push((current_item, current_length));
        }

        encoded
    }

    /// Encode vector content using run length encoding, moving the values
    /// in the process.
    ///
    /// Identical to `run_length_encoding`, except values are moved to the
    /// returned vector.
    ///
    /// Run length encoding is a compression algorithm where each item is
    /// stored as a value and length, such that sequences of identical values
    /// are only stored as a single item.
    ///
    /// Source: https://en.wikipedia.org/wiki/Run-length_encoding
    ///
    /// # Examples
    /// Example 1
    /// ```
    /// use terminal_renderer::utils::VecUtils;
    ///
    /// let vec = vec![0, 0, 0, 3, 1, 1, 5];
    ///
    /// assert_eq!(vec.into_run_length_encoding(), vec![(0, 3), (3, 1), (1, 2), (5, 1)]);
    /// ```
    ///
    /// Example 2
    /// ```
    /// use terminal_renderer::utils::VecUtils;
    ///
    /// let chars = vec!['a', 'a', 'a', 'b', 'b', 'c', 'd', 'd', 'd', 'd'];
    ///
    /// assert_eq!(chars.into_run_length_encoding(), vec![('a', 3), ('b', 2), ('c', 1), ('d', 4)]);
    /// ```
    fn into_run_length_encoding(self) -> Vec<(T, usize)> {
        let mut encoded = Vec::new();

        let mut last_item = None;
        let mut current_length = 0;
        for item in self {
            let Some(ref current_item) = last_item else {
                last_item = Some(item);
                current_length = 1;
                continue;
            };

            if current_item != &item {
                encoded.push((last_item.unwrap(), current_length));
                last_item = Some(item);
                current_length = 0;
            }

            current_length += 1;
        }

        if let Some(current_item) = last_item {
            encoded.push((current_item, current_length));
        }

        encoded
    }
}

pub struct RunLengthEncoded<'a, I: 'a, T> {
    last_item: Option<&'a T>,
    iter: I,
}

impl<'a, I, T> RunLengthEncoded<'a, I, T>
where
    I: Iterator<Item = &'a T>,
{
    pub fn new(mut iter: I) -> Self {
        Self {
            last_item: iter.next(),
            iter,
        }
    }
}

impl<'a, I, T> Iterator for RunLengthEncoded<'a, I, T>
where
    I: Iterator<Item = &'a T>,
    T: PartialEq,
{
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.last_item?;

        let mut count = 1_usize;

        loop {
            self.last_item = self.iter.next();
            let Some(next_item) = &self.last_item else {
                break;
            };
            if next_item != &item {
                break;
            }

            count += 1;
        }

        Some((item, count))
    }
}

pub trait IterUtils<'a, T>: Iterator<Item = &'a T>
where
    Self: Sized,
    T: PartialEq + 'a,
{
    fn run_length_encoded(self) -> RunLengthEncoded<'a, Self, T>;
}

impl<'a, I, T> IterUtils<'a, T> for I
where
    I: Iterator<Item = &'a T>,
    T: PartialEq + 'a,
{
    fn run_length_encoded(self) -> RunLengthEncoded<'a, Self, T> {
        RunLengthEncoded::new(self)
    }
}

pub fn str_count_lines(string: &str) -> usize {
    string.chars().filter(|char| *char == '\n').count()
}

// Requires rand crate
// pub fn random_bool_probability(probability: f32) -> bool {
//     rand::random::<f32>() < probability
// }

pub enum MaybeBorrowed<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}
pub enum MaybeBorrowedMut<'a, T> {
    BorrowedMut(&'a mut T),
    OwnedMut(T),
}

pub use self::MaybeBorrowed::*;
pub use self::MaybeBorrowedMut::*;

pub struct PeekableReceiver<T> {
    receiver: mpsc::Receiver<T>,
    peeked: Option<T>,
}

impl<T> PeekableReceiver<T> {
    pub fn new(receiver: mpsc::Receiver<T>) -> Self {
        Self {
            receiver,
            peeked: None,
        }
    }

    pub fn try_peeked(&mut self) -> Option<T> {
        // Swap from here: https://stackoverflow.com/a/63354217/15507414
        let mut value = None;
        swap(&mut self.peeked, &mut value);
        value
    }

    pub fn is_empty(&mut self) -> Result<bool, mpsc::RecvError> {
        if self.peeked.is_some() {
            return Ok(false);
        }

        match self.receiver.try_recv() {
            Ok(value) => {
                self.peeked = Some(value);
                Ok(false)
            }
            Err(mpsc::TryRecvError::Empty) => Ok(true),
            Err(_) => Err(mpsc::RecvError),
        }
    }

    /// Attempts to wait for a value on this receiver, returning an error if the
    /// corresponding channel has hung up.
    ///
    /// This function will always block the current thread if there is no data
    /// available and it's possible for more data to be sent (at least one sender
    /// still exists). Once a message is sent to the corresponding [`Sender`]
    /// (or [`SyncSender`]), this receiver will wake up and return that
    /// message.
    ///
    /// If the corresponding [`Sender`] has disconnected, or it disconnects while
    /// this call is blocking, this call will wake up and return [`Err`] to
    /// indicate that no more messages can ever be received on this channel.
    /// However, since channels are buffered, messages sent before the disconnect
    /// will still be properly received.
    pub fn recv(&mut self) -> Result<T, mpsc::RecvError> {
        if let Some(value) = self.try_peeked() {
            return Ok(value);
        };

        self.receiver.recv()
    }

    /// Attempts to return a pending value on this receiver without blocking.
    ///
    /// This method will never block the caller in order to wait for data to
    /// become available. Instead, this will always return immediately with a
    /// possible option of pending data on the channel.
    ///
    /// This is useful for a flavor of "optimistic check" before deciding to
    /// block on a receiver.
    ///
    /// Compared with [`recv`], this function has two failure cases instead of one
    /// (one for disconnection, one for an empty buffer).
    ///
    /// [`recv`]: Self::recv
    pub fn try_recv(&mut self) -> Result<T, mpsc::TryRecvError> {
        if let Some(value) = self.try_peeked() {
            return Ok(value);
        };

        self.receiver.try_recv()
    }

    /// Attempts to wait for a value on this receiver, returning an error if the
    /// corresponding channel has hung up, or if it waits more than `timeout`.
    ///
    /// This function will always block the current thread if there is no data
    /// available and it's possible for more data to be sent (at least one sender
    /// still exists). Once a message is sent to the corresponding [`Sender`]
    /// (or [`SyncSender`]), this receiver will wake up and return that
    /// message.
    ///
    /// If the corresponding [`Sender`] has disconnected, or it disconnects while
    /// this call is blocking, this call will wake up and return [`Err`] to
    /// indicate that no more messages can ever be received on this channel.
    /// However, since channels are buffered, messages sent before the disconnect
    /// will still be properly received.
    ///
    /// # Known Issues
    ///
    /// There is currently a known issue (see [`#39364`]) that causes `recv_timeout`
    /// to panic unexpectedly with the following example:
    pub fn recv_timeout(&mut self, timeout: Duration) -> Result<T, mpsc::RecvTimeoutError> {
        if let Some(value) = self.try_peeked() {
            return Ok(value);
        };

        self.receiver.recv_timeout(timeout)
    }

    // pub fn iter(&self) -> mpsc::Iter<'_, T> {
    //     self.receiver.iter()
    // }

    // pub fn try_iter(&self) -> mpsc::TryIter<'_, T> {
    //     self.receiver.try_iter()
    // }
}

pub trait GenericSum: Iterator {
    fn generic_sum(self) -> Self::Item
    where
        Self: Sized,
        Self::Item: super::math::Num,
    {
        self.reduce(|accum, item| accum + item).unwrap_or_else(zero)
    }
}

impl<T: ?Sized> GenericSum for T where T: Iterator {}

#[derive(Debug)]
pub struct Ref<T> {
    id: u32,
    phantom: PhantomData<T>,
}

pub struct RefVec<T> {
    vec: Vec<(u32, Option<T>)>,
    next_id: u32,
}

impl<T> RefVec<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            next_id: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Ref<T> {
        self.vec.push((self.next_id, Some(item)));

        let reference = Ref {
            id: self.next_id,
            phantom: PhantomData::<T>,
        };

        self.next_id += 1;

        reference
    }

    pub fn find_index(&self, reference: &Ref<T>) -> Option<usize> {
        if reference.id >= self.next_id {
            return None;
        }

        let mut top = self.vec.len() - 1;
        let mut bottom = 0;

        let index = loop {
            let middle = (top + bottom) / 2;

            match self.vec[middle].0.cmp(&reference.id) {
                std::cmp::Ordering::Less => bottom = middle + 1,
                std::cmp::Ordering::Greater => top = middle - 1,
                std::cmp::Ordering::Equal => break middle,
            }
        };

        self.vec[index].1.as_ref()?;

        Some(index)
    }

    pub fn get(&self, reference: &Ref<T>) -> Option<&T> {
        let (id, item) = self.vec.get(self.find_index(reference)?)?;

        if *id != reference.id {
            return None;
        }

        item.as_ref()
    }

    pub fn get_mut(&mut self, reference: &Ref<T>) -> Option<&mut T> {
        let index = self.find_index(reference)?;
        let (id, item) = self.vec.get_mut(index)?;

        if *id != reference.id {
            return None;
        }

        item.as_mut()
    }

    pub fn get_index(&self, index: usize) -> Option<&T> {
        (self.vec.get(index)?.1).as_ref()
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut T> {
        (self.vec.get_mut(index)?.1).as_mut()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn insert(&mut self, index: usize, element: T) -> Ref<T> {
        self.vec.insert(index, (self.next_id, Some(element)));

        let reference = Ref {
            id: self.next_id,
            phantom: PhantomData::<T>,
        };

        self.next_id += 1;

        reference
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()?.1
    }

    pub fn remove(&mut self, reference: &Ref<T>) -> Option<T> {
        self.remove_index(self.find_index(reference)?)
    }

    pub fn remove_index(&mut self, index: usize) -> Option<T> {
        self.vec.remove(index).1
    }

    pub fn iter(&self) -> AnyIter<&T> {
        let iter = self.vec.iter().filter_map(|(_, item)| item.as_ref());
        AnyIter::new(iter)
    }
    pub fn iter_mut(&mut self) -> AnyIter<&mut T> {
        let iter = self.vec.iter_mut().filter_map(|(_, item)| item.as_mut());
        AnyIter::new(iter)
    }
}

impl<T> Default for RefVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Debug for RefVec<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.vec.fmt(f)
    }
}

// pub struct DynRefVec<T>
// where
//     T: ?Sized,
// {
//     vec: Vec<(u32, Option<Box<T>>)>,
//     next_id: u32,
// }

// impl<T> DynRefVec<T>
// where
//     T: ?Sized,
// {
//     pub fn new() -> Self {
//         Self {
//             vec: Vec::new(),
//             next_id: 0,
//         }
//     }

//     pub fn push<U>(&mut self, item: U) -> Ref<U> {
//         self.vec.push((self.next_id, Some(Box::new(item))));

//         let reference = Ref {
//             id: self.next_id,
//             phantom: PhantomData::<U>,
//         };

//         self.next_id += 1;

//         reference
//     }
// }

pub struct AnyIter<'a, T> {
    iter: Box<dyn Iterator<Item = T> + 'a>,
}

impl<'a, T> AnyIter<'a, T> {
    pub fn new(iter: impl Iterator<Item = T> + 'a) -> Self {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<'a, T> Iterator for AnyIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct DoubleEndedAnyIter<'a, T> {
    iter: Box<dyn DoubleEndedIterator<Item = T> + 'a>,
}

impl<'a, T> DoubleEndedAnyIter<'a, T> {
    pub fn new(iter: impl DoubleEndedIterator<Item = T> + 'a) -> Self {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<'a, T> Iterator for DoubleEndedAnyIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> DoubleEndedIterator for DoubleEndedAnyIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

#[derive(Debug)]
pub struct DeltaTimer {
    last_time: Instant,
}

impl DeltaTimer {
    pub fn new() -> Self {
        let last_time = Instant::now();
        Self { last_time }
    }

    pub fn delta_time(&mut self) -> Duration {
        let now = Instant::now();

        let duration = now.duration_since(self.last_time);
        // .expect("time went backwards");
        self.last_time = now;

        duration
    }

    pub fn delta_s(&mut self) -> f32 {
        let duration = self.delta_time();
        duration.as_secs_f32()
    }

    pub fn restart(&mut self) {
        self.last_time = Instant::now();
    }
}

impl Default for DeltaTimer {
    fn default() -> Self {
        Self::new()
    }
}
