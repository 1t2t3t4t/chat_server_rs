use std::collections::HashMap;

use tokio::sync::mpsc::UnboundedSender;

use crate::schema::Event;
use tokio::sync::mpsc::error::SendError;

pub trait Controller: Send {
    fn register(&mut self, user: &str, tx: UnboundedSender<Event>);
    fn send(&self, event: Event, to_user: &str) -> Result<(), SendError<Event>>;
}

#[derive(Default)]
pub(crate) struct InMemController {
    users: HashMap<String, UnboundedSender<Event>>
}

impl Controller for InMemController {
    fn register(&mut self, user: &str, tx: UnboundedSender<Event>) {
        self.users.insert(user.to_string(), tx);
    }

    fn send(&self, event: Event, to_user: &str) -> Result<(), SendError<Event>> {
        if let Some(tx) = self.users.get(to_user) {
            return tx.send(event);
        }
        Ok(())
    }
}
