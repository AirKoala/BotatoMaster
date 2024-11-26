use super::selector::Selector;
use super::responder::Responder;
use serenity::{model::channel::Message, prelude::*};

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
    pub fn new(selectors: Vec<Selector>, responder:  BoxedResponder) -> Self {
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

pub fn load_matchers() -> Vec<ContentMatcher> {
    vec![
        ContentMatcher::new(
            vec![Selector::default().regex_str("hello").expect("Invalid regex")],
            Box::new(super::responder::SimpleResponder::new(vec![
                "Hello!".to_string(),
                "Hi!".to_string(),
                "Hey!".to_string(),
            ])),
        ),
        ContentMatcher::new(
            vec![Selector::default().regex_str("goodbye").expect("Invalid regex")],
            Box::new(super::responder::SimpleResponder::new(vec![
                "Goodbye!".to_string(),
                "Bye!".to_string(),
                "See you later!".to_string(),
            ])),
        ),
    ]
}
