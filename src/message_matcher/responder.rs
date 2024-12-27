use eyre::Result;
use rand::seq::SliceRandom;
use serenity::{async_trait, model::channel::Message, prelude::*};

#[async_trait()]
pub trait Responder {
    async fn respond(&self, context: &Context, message: &Message) -> Result<()>;
}

struct Response {
    pub content: String,
    pub bias: u64,
}

pub struct SimpleResponder {
    responses: Vec<Response>,
}
impl SimpleResponder {
    pub fn from_tuples(responses: Vec<(String, u64)>) -> Self {
        let responses = responses
            .into_iter()
            .map(|(content, bias)| Response { content, bias })
            .collect();

        SimpleResponder { responses }
    }
}

#[async_trait()]
impl Responder for SimpleResponder {
    async fn respond(&self, context: &Context, message: &Message) -> Result<()> {
        // Choose a response at random to send.
        let chosen_response = self.responses.choose(&mut rand::thread_rng()).unwrap();

        // TODO: Implement biasing.
        message.channel_id.say(&context.http, &chosen_response.content).await?;

        Ok(())
    }
}
