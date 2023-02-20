use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::Sum;
use std::time::Instant;
use std::{any::*, ops};
// use vector_map::VecMap;

type ValuesMap<T> = HashMap<ValueId__, T>;

const MOVING_AVERAGE_COUNT: usize = 20;

thread_local! {
    static VALUES: RefCell<ValuesMap<Box<dyn Any>>> = RefCell::new(HashMap::new());
    static AVERAGES: RefCell<ValuesMap<(usize, [f64; MOVING_AVERAGE_COUNT])>> = RefCell::new(HashMap::new());
    static DURATIONS: RefCell<IndexMap<&'static str, MovingSmartAverage<f64, MOVING_AVERAGE_COUNT>>> = RefCell::new(IndexMap::new());
    static TOTAL_DURATION: RefCell<(Instant, MovingSmartAverage<f64, MOVING_AVERAGE_COUNT>)> = RefCell::new((Instant::now(), MovingSmartAverage::new()));
    static DURATIONS_TIME: RefCell<Instant> = RefCell::new(Instant::now());
}

const SCALE_DIFFERENCE_THRESHOLD: usize = 4;
pub struct MovingSmartAverage<T, const SIZE: usize>
where
    T: ops::Add<Output = T>
        + ops::Div<Output = T>
        + PartialOrd<T>
        + num::FromPrimitive
        + num::Zero
        + Copy
        + Sum<T>,
{
    next_index: usize,
    values: [T; SIZE],
}

impl<T, const SIZE: usize> MovingSmartAverage<T, SIZE>
where
    T: ops::Add<Output = T>
        + ops::Div<Output = T>
        + PartialOrd<T>
        + num::FromPrimitive
        + num::Zero
        + Copy
        + Sum<T>,
{
    pub fn new() -> Self {
        Self {
            next_index: 0,
            values: [T::zero(); SIZE],
        }
    }

    pub fn insert(&mut self, value: T) {
        let old_average = self.average();
        let scale_difference = value / old_average;
        if scale_difference
            >= T::from_usize(SCALE_DIFFERENCE_THRESHOLD).expect("to large threshold")
        {
            self.values = [value; SIZE];
            return;
        }

        self.values[self.next_index] = value;
        self.next_index += 1;
        if self.next_index >= SIZE {
            self.next_index = 0;
        }
    }

    pub fn average(&self) -> T {
        self.values.iter().cloned().sum::<T>() / T::from_usize(SIZE).expect("to large threshold")
    }
}

impl<T, const SIZE: usize> Default for MovingSmartAverage<T, SIZE>
where
    T: ops::Add<Output = T>
        + ops::Div<Output = T>
        + PartialOrd<T>
        + num::FromPrimitive
        + num::Zero
        + Copy
        + Sum<T>,
{
    fn default() -> Self {
        Self::new()
    }
}

/// This is an internal type, which should not be used directly.
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
#[doc(hidden)]
pub struct ValueId__ {
    pub label: &'static str,
    pub line: u32,
    pub column: u32,
    pub file: &'static str,
}

/// This is an implementation detail, which should not be called directly.
///
/// This function should instead be called using the `has_value_changed!()` macro.
#[doc(hidden)]
pub fn has_value_changed_internal__<T>(value: &T, id: ValueId__) -> bool
where
    T: 'static + PartialEq + Debug + Clone,
{
    // file!()

    VALUES.with(|values| {
        let mut values = values.borrow_mut();

        let has_changed = match values.get(&id) {
            Some(old_value_box) => match old_value_box.downcast_ref::<T>() {
                Some(old_value) => old_value != value,
                None => true,
            },
            None => true,
        };

        let value_clone = value.to_owned();

        values.insert(id, Box::new(value_clone));

        has_changed
    })
}

#[macro_export]
macro_rules! has_value_changed {
    ($value:expr) => {
        $crate::rasmus_lib::debug_utils::has_value_changed_internal__(
            $value,
            $crate::get_value_id!(),
        )
    };
}

#[macro_export]
macro_rules! dbg_value_changed {
    () => {
        dbg!();
    };
    ($value:expr) => {
        if $crate::has_value_changed!($value) {
            dbg!($value);
        }
    };
}

/// This is an implementation detail, which should not be called directly.
///
/// This function should instead be called using the `value_changed!()` macro.
#[doc(hidden)]
pub fn update_moving_average_internal__<T>(value: T, id: ValueId__) -> f64
where
    T: Into<f64>,
{
    let value = value.into();
    // file!()

    AVERAGES.with(|values| {
        let mut values = values.borrow_mut();

        let slot = match values.get_mut(&id) {
            Some(slot) => slot,
            None => {
                values.insert(id, (0, [value; MOVING_AVERAGE_COUNT]));
                values.get_mut(&id).unwrap()
            }
        };

        slot.1[slot.0] = value;
        slot.0 += 1;
        if slot.0 >= MOVING_AVERAGE_COUNT {
            slot.0 = 0;
        }

        slot.1.iter().sum::<f64>() / MOVING_AVERAGE_COUNT as f64
    })
}

#[macro_export]
macro_rules! get_value_id {
    () => {
        $crate::rasmus_lib::debug_utils::ValueId__ {
            label: "",
            line: line!(),
            column: column!(),
            file: file!(),
        }
    };
    ($label:expr) => {
        $crate::rasmus_lib::debug_utils::ValueId__ {
            label: $label,
            line: line!(),
            column: column!(),
            file: file!(),
        }
    };
}

#[macro_export]
macro_rules! update_moving_average {
    ($value:expr) => {
        $crate::rasmus_lib::debug_utils::update_moving_average_internal__(
            $value,
            $crate::get_value_id!(),
        )
    };
}

#[macro_export]
macro_rules! update_moving_average_label {
    ($label:expr, $value:expr) => {
        $crate::rasmus_lib::debug_utils::update_moving_average_internal__(
            $value,
            $crate::get_value_id!($label),
        )
    };
}

pub fn start_new_timer_frame() {
    DURATIONS.with(|durations| {
        TOTAL_DURATION.with(|total| {
            let durations = durations.borrow();
            let mut total = total.borrow_mut();

            let last_time = total.0;
            let now = Instant::now();
            DURATIONS_TIME.with(|time| {
                *time.borrow_mut() = now;
            });

            let current_total_milis = now.duration_since(last_time).as_millis() as f64;
            total.0 = now;

            total.1.insert(current_total_milis);

            let average_total_milis = total.1.average();

            for duration in &*durations {
                let label = *duration.0;
                let milis = duration.1.average();

                print!("{}: {:.2} ms, ", label, milis);
            }

            println!("| total frame: {:.2} ms", average_total_milis);
        })
    });
}

pub fn reset_timer() {
    DURATIONS_TIME.with(|time| {
        *time.borrow_mut() = Instant::now();
    });
}

pub fn update_timer_label(label: &'static str) {
    DURATIONS.with(|durations| {
        DURATIONS_TIME.with(|time| {
            let mut durations = durations.borrow_mut();
            let mut time = time.borrow_mut();
            let now = Instant::now();

            let milis = now.duration_since(*time).as_millis() as f64;
            *time = now;

            let entry = durations.entry(label);
            entry
                .and_modify(|average_buffer| average_buffer.insert(milis))
                .or_insert(MovingSmartAverage::new());
        });
    });
}
