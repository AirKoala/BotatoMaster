use crate::message_matcher::matcher::ContentMatcher;
use chrono::{DateTime, Utc};
use eyre::{Context, OptionExt, Result};
use serenity::model::id::UserId;
use sqlx::SqlitePool;

use super::{
    response::{fetch_responses_for_matcher, simple_responder_from_models},
    selector::{fetch_selectors_for_matcher, selectors_from_models},
};

#[derive(Debug)]
struct ContentMatcherModel {
    pub id: i64,
    pub approved: i64,
    pub author_id: i64,
    pub added_on: i64,
}
impl ContentMatcherModel {
    pub async fn into_content_matcher(self, pool: &SqlitePool) -> Result<ContentMatcher> {
        // TODO: Do something with author_id and added_on

        let selectors = selectors_from_models(fetch_selectors_for_matcher(self.id, pool).await?)?;
        let responder = Box::new(simple_responder_from_models(
            fetch_responses_for_matcher(self.id, pool).await?,
        ));

        let matcher = ContentMatcher::new(selectors, responder);

        Ok(matcher)
    }
}

pub async fn fetch_all_approved_matchers(pool: &SqlitePool) -> Result<Vec<ContentMatcherModel>> {
    sqlx::query_as!(
        ContentMatcherModel,
        "SELECT * FROM content_matcher
        WHERE approved = TRUE",
    )
    .fetch_all(pool)
    .await
    .wrap_err("Failed to fetch all content matchers.")
}
