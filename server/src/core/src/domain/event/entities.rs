use crate::domain::prelude::Person;

pub trait Event: Send + Sync + 'static {}

pub struct PersonCreatedEvent {
    pub person: Person,
}

impl Event for PersonCreatedEvent {}

// use std::sync::{Arc, RwLock};
// use std::any::{Any, TypeId};
// use std::collections::HashMap;

// type HandlerFn = Arc<
//     dyn Fn(&dyn Any) -> Pin<Box<dyn Future<Output = ()> + Send>>
//         + Send
//         + Sync
// >;

// pub struct InMemoryEventBus {
//     handlers: RwLock<HashMap<TypeId, Vec<HandlerFn>>>,
// }

// impl InMemoryEventBus {
//     pub fn new() -> Self {
//         Self {
//             handlers: RwLock::new(HashMap::new()),
//         }
//     }

//     pub fn subscribe<E, H>(&self, handler: H)
//     where
//         E: Event,
//         H: EventHandler<E> + 'static,
//     {
//         let mut map = self.handlers.write().unwrap();

//         let entry = map.entry(TypeId::of::<E>()).or_default();

//         let handler = Arc::new(handler);

//         entry.push(Arc::new(move |event: &dyn Any| {
//             let event = event.downcast_ref::<E>().unwrap();
//             handler.handle(event)
//         }));
//     }
// }

// impl EventBus for InMemoryEventBus {
//     fn publish<E: Event>(
//         &self,
//         event: E,
//     ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
//         let handlers = {
//             let map = self.handlers.read().unwrap();
//             map.get(&TypeId::of::<E>())
//                 .cloned()
//                 .unwrap_or_default()
//         };

//         Box::pin(async move {
//             for handler in handlers {
//                 handler(&event).await;
//             }
//         })
//     }
// }
