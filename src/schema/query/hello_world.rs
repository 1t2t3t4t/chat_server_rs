use crate::schema::Message;
use crate::CookieSetter;
use async_graphql::Context;
use std::sync::Arc;

#[derive(Default)]
pub struct HelloWorldQuery;

#[async_graphql::Object]
impl HelloWorldQuery {
    pub async fn hello_world(&self, ctx: &Context<'_>) -> Vec<Message> {
        let cookie_setter = ctx.data_unchecked::<Arc<CookieSetter>>();
        let mut lock_cookies = cookie_setter.cookies.try_lock().unwrap();
        lock_cookies.insert("username".into(), "Some name".into());
        (0..100)
            .into_iter()
            .map(|i| Message {
                msg: format!("Hello no {}", i),
                sender: "System".into(),
            })
            .collect()
    }
}
