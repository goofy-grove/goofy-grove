mod handlers;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    pin::Pin,
    sync::{Arc, RwLock},
};

use gg_core::domain::prelude::*;

pub use handlers::*;

type HandlerFn = Arc<dyn Fn(&dyn Any) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

#[derive(Clone)]
pub struct InMemoryEventBus {
    handlers: Arc<RwLock<HashMap<TypeId, Vec<HandlerFn>>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl EventPublisher for InMemoryEventBus {
    fn publish<E: Event>(&self, event: E) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let handlers = self.handlers.read().unwrap();
        let handlers = handlers.get(&TypeId::of::<E>()).cloned();

        Box::pin(async move {
            if let Some(handlers) = handlers {
                for handler in handlers {
                    handler(&event).await;
                }
            }
        })
    }
}

impl EventSubscriber for InMemoryEventBus {
    fn subscribe<E: Event, H: EventHandler<E> + 'static>(&self, handler: H) {
        let mut map = self.handlers.write().unwrap();
        let entry = map.entry(TypeId::of::<E>()).or_default();
        let handler = Arc::new(handler);

        entry.push(Arc::new(move |event: &dyn Any| {
            let event = event.downcast_ref::<E>().unwrap();
            handler.handle(event)
        }));
    }
}
