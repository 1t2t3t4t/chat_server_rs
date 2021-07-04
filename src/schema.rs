use crate::controller::{Controller, InMemController};
use async_graphql::{
    Context, InputObject, MergedObject, Object, Schema, SimpleObject, Subscription, Union,
};
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::Mutex;

use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::{Stream, StreamExt};

#[derive(Default)]
pub struct HelloWorldQuery;

#[Object]
impl HelloWorldQuery {
    async fn hello_world(&self) -> String {
        "Hello World".into()
    }
}

#[derive(MergedObject, Default)]
pub struct Query(HelloWorldQuery);

pub struct Mutation;

#[Object]
impl Mutation {
    async fn send_msg(&self, ctx: &Context<'_>, msg: String, to: String) -> bool {
        let controller = ctx.data::<Mutex<Box<dyn Controller>>>().unwrap();
        let lock = controller.lock().await;
        let new_msg = Message {
            msg,
            sender: "SomeOne".to_string(),
        };
        if let Ok(_) = lock.send(Event::Message(new_msg), to) {
            true
        } else {
            false
        }
    }
}

#[derive(InputObject)]
pub struct JoinRequest {
    name: String,
    age: i32,
}

#[derive(SimpleObject, Debug)]
pub struct Message {
    msg: String,
    sender: String,
}

#[derive(Union, Debug)]
pub enum Event {
    Message(Message),
}

pub struct MySubscription;

#[Subscription]
impl MySubscription {
    async fn join(&self, ctx: &Context<'_>, req: JoinRequest) -> impl Stream<Item = Event> {
        println!("Get Join");
        let in_mem_controller = ctx.data::<Mutex<Box<dyn Controller>>>();
        let mut lock = in_mem_controller.unwrap().try_lock().unwrap();
        let (tx, rx) = unbounded_channel::<Event>();
        lock.register(req.name, tx);
        UnboundedReceiverStream::new(rx)
    }
}

pub type MySchema = Schema<Query, Mutation, MySubscription>;
