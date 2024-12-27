use crate::database::models::content_matcher::fetch_all_approved_matchers;

use super::responder::Responder;
use super::selector::Selector;
use eyre::Result;
use serenity::{model::channel::Message, prelude::*};
use sqlx::SqlitePool;

pub enum MatchResult {
    Matched,
    NoMatch,
    // Error,
}

type BoxedResponder = Box<dyn Responder + Send + Sync>;

pub struct ContentMatcher {
    selectors: Vec<Selector>,
    responder: BoxedResponder,
}

impl ContentMatcher {
    pub fn new(selectors: Vec<Selector>, responder: BoxedResponder) -> Self {
        ContentMatcher {
            selectors,
            responder,
        }
    }

    fn matches(&self, _context: &Context, message: &Message) -> bool {
        for selector in &self.selectors {
            if selector.matches(message) {
                return true;
            }
        }
        false
    }

    async fn respond(&self, context: &Context, message: &Message) {
        if let Err(error) = self.responder.respond(&context, &message).await {
            println!("Error sending message: {:?}", error);
        }
    }

    pub async fn handle(&self, context: &Context, message: &Message) -> MatchResult {
        println!("Matchers: {:?}", self.selectors);
        println!("Handling message: {:?}", message.content);
        if self.matches(&context, message) {
            self.respond(&context, message).await;
            return MatchResult::Matched;
        }
        return MatchResult::NoMatch;
    }
}

pub async fn load_matchers(pool: &SqlitePool) -> Result<Vec<ContentMatcher>> {
    let mut matchers = Vec::new();

    for matcher in fetch_all_approved_matchers(pool).await? {
        let id = matcher.id;

        match matcher.into_content_matcher(pool).await {
            Ok(matcher) => matchers.push(matcher),
            Err(error) => println!("Error loading matcher id {}: {:?}", id, error),
        }
    }

    Ok(matchers)
}
