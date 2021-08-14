use crate::schema::Message;

#[derive(Default)]
pub struct HelloWorldQuery;

#[async_graphql::Object]
impl HelloWorldQuery {
    pub async fn hello_world(&self) -> Vec<Message> {
        (0..100)
            .into_iter()
            .map(|i| Message {
                msg: format!("Hello no {}", i),
                sender: "System".into(),
            })
            .collect()
    }
}
