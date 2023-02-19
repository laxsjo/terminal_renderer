use std::any::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
// use vector_map::VecMap;

type ValuesMap<T> = HashMap<ValueId__, T>;

const MOVING_AVERAGE_COUNT: usize = 20;

thread_local! {
    static VALUES: RefCell<ValuesMap<Box<dyn Any>>> = RefCell::new(HashMap::new());
    static AVERAGES: RefCell<ValuesMap<(usize, [f64; MOVING_AVERAGE_COUNT])>> = RefCell::new(HashMap::new());
}

/// This is an internal type, which should not be used directly.
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
#[doc(hidden)]
pub struct ValueId__ {
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
            $crate::rasmus_lib::debug_utils::ValueId__ {
                line: line!(),
                column: column!(),
                file: file!(),
            },
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
macro_rules! update_moving_average {
    ($value:expr) => {
        $crate::rasmus_lib::debug_utils::update_moving_average_internal__(
            $value,
            $crate::rasmus_lib::debug_utils::ValueId__ {
                line: line!(),
                column: column!(),
                file: file!(),
            },
        )
    };
}

// pub(crate) use value_changed;
