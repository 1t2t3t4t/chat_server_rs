use std::collections::HashMap;

use tokio::sync::mpsc::UnboundedSender;

use crate::schema::Event;

pub trait Controller {
    fn register(&mut self, user: String, tx: UnboundedSender<Event>);
    fn send(&self, event: Event, to_user: String);
}

#[derive(Default)]
pub struct InMemController {
    users: HashMap<String, UnboundedSender<Event>>,
}

impl Controller for InMemController {
    fn register(&mut self, user: String, tx: UnboundedSender<Event>) {
        self.users.insert(user, tx);
    }

    fn send(&self, event: Event, to_user: String) {
        if let Some(tx) = self.users.get(&to_user) {
            tx.send(event);
        }
    }
}
