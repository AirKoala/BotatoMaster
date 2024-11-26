mod matcher;
mod responder;
mod selector;

use matcher::{load_matchers, ContentMatcher, MatchResult};

use serenity::{async_trait, model::channel::Message, prelude::*};

pub struct MessageMatcherHandler {
    matchers: Vec<ContentMatcher>,
}
impl MessageMatcherHandler {
    pub fn new() -> Self {
        MessageMatcherHandler { matchers: load_matchers() }
    }
}

#[async_trait]
impl EventHandler for MessageMatcherHandler {
    async fn message(&self, context: Context, message: Message) {
        for matcher in &self.matchers {
            if let MatchResult::Matched = matcher.handle(&context, &message).await {
                break;
            }
        }
    }
}
