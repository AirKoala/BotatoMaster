use anyhow::Result;
use rand::seq::SliceRandom;
use serenity::{async_trait, model::channel::Message, prelude::*};

#[async_trait()]
pub trait Responder {
    async fn respond(&self, context: &Context, message: &Message) -> Result<()>;
}

pub struct SimpleResponder {
    responses: Vec<String>,
}
impl SimpleResponder {
    pub fn new(responses: Vec<String>) -> Self {
        SimpleResponder { responses }
    }
}

#[async_trait()]
impl Responder for SimpleResponder {
    async fn respond(&self, context: &Context, message: &Message) -> Result<()> {
        // Choose a response at random to send.
        let chosen_response = self.responses.choose(&mut rand::thread_rng()).unwrap();

        message.channel_id.say(&context.http, chosen_response).await?;

        Ok(())
    }
}
