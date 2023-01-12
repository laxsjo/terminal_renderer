// use std::sync::mpsc::Receiver;

use crate::utils::Ref;
use crate::utils::RefVec;

pub trait EventData {}
pub struct Event<T, R>
where
    T: EventData,
    R: EventReceiver,
{
    data: T,
    receiver: Ref<R>,
}

impl<T, R> Event<T, R>
where
    T: EventData,
    R: EventReceiver,
{
    pub fn new(receiver: Ref<R>, data: T) -> Self {
        Self { data, receiver }
    }

    pub fn data(self) -> T {
        self.data
    }
}

pub trait EventHandler<T, R>
where
    T: EventData,
    R: EventReceiver,
{
    fn handle(&self, receiver: &mut R, data: T);
}

pub trait EventReceiver
where
    Self: Sized,
{
    fn receive_event<T: EventData>(&mut self, event: Event<T, Self>);
}

pub trait EventManager<R>
where
    R: EventReceiver,
{
    fn get_items(&mut self) -> &mut RefVec<R>;

    fn send_event<T: EventData>(&mut self, event: Event<T, R>) -> Option<()> {
        let items = self.get_items();

        let item = items.get_mut(&event.receiver)?;

        item.receive_event(event);

        Some(())
    }
}
