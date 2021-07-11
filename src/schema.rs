use std::io::Read;

use crate::controller::Controller;
use async_graphql::{Context, InputObject, MergedObject, Object, Schema, SimpleObject, Subscription, Union, Upload};
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::Mutex;

use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::Stream;

#[derive(Default)]
pub struct HelloWorldQuery;

#[Object]
impl HelloWorldQuery {
    async fn hello_world(&self) -> Vec<Message> {
        (0..100)
            .into_iter()
            .map(|i| Message {
                msg: format!("Hello no {}", i),
                sender: "System".into(),
            })
            .collect()
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
        if let Ok(_) = lock.send(Event::Message(new_msg), &to) {
            true
        } else {
            false
        }
    }

    async fn upload_file(&self, ctx: &Context<'_>, file: Upload) -> String {
        let file_val = file.value(ctx).unwrap();
        let mut file = file_val.content;
        let metadata = file.metadata().unwrap();
        let mut file_bytes = vec![0; metadata.len() as usize];
        file.read(&mut file_bytes).unwrap();
        std::fs::write(&file_val.filename, file_bytes).unwrap();
        file_val.filename
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

#[derive(SimpleObject, Debug)]
pub struct Connected {
    msg: String,
}

#[derive(Union, Debug)]
pub enum Event {
    Message(Message),
    Connected(Connected),
}

pub struct MySubscription;

#[Subscription]
impl MySubscription {
    async fn join(&self, ctx: &Context<'_>, req: JoinRequest) -> impl Stream<Item = Event> {
        println!("Get Join");
        let in_mem_controller = ctx.data::<Mutex<Box<dyn Controller>>>();
        let mut lock = in_mem_controller.unwrap().try_lock().unwrap();
        let (tx, rx) = unbounded_channel::<Event>();
        lock.register(&req.name, tx);
        lock.send(
            Event::Connected(Connected {
                msg: "Welcome to the server!!".into(),
            }),
            &req.name,
        )
        .unwrap();
        UnboundedReceiverStream::new(rx)
    }
}

pub type MySchema = Schema<Query, Mutation, MySubscription>;
