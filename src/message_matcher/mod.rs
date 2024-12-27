pub mod matcher;
pub mod responder;
pub mod selector;

use matcher::{load_matchers, ContentMatcher, MatchResult};

use eyre::Result;
use serenity::{async_trait, model::channel::Message, prelude::*};
use sqlx::SqlitePool;

pub struct MessageMatcherHandler {
    matchers: Vec<ContentMatcher>,
}
impl MessageMatcherHandler {
    pub async fn new(pool: &SqlitePool) -> Result<Self> {
        Ok(MessageMatcherHandler {
            matchers: load_matchers(pool).await?,
        })
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
