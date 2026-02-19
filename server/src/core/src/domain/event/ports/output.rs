use std::pin::Pin;

use crate::domain::prelude::*;

pub trait EventPublisher: Send + Sync {
    fn publish<E: Event>(&self, event: E) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

pub trait EventHandler<E: Event>: Send + Sync {
    fn handle(&self, event: &E) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

pub trait EventSubscriber: Send + Sync {
    fn subscribe<E: Event, H: EventHandler<E> + 'static>(&self, handler: H);
}
