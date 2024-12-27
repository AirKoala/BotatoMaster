use eyre::{Context, Result};
use sqlx::SqlitePool;
use serenity::model::id::*;

use crate::message_matcher::selector::Selector;

#[derive(Debug, sqlx::FromRow)]
pub struct SelectorModel {
    id: i64,
    content_matcher_id: i64,
    regex: Option<String>,
    author_id: Option<i64>,
    channel_id: Option<i64>,
    guild_id: Option<i64>,
}
impl SelectorModel {
    pub fn into_selector(self) -> Result<Selector> {
        let mut selector = Selector::default();

        if let Some(regex) = self.regex {
            selector.regex_str(&regex)?;
        }

        if let Some(author_id) = self.author_id {
            selector.author(UserId::new(author_id.try_into()?));
        }

        if let Some(channel_id) = self.channel_id {
            selector.channel(ChannelId::new(channel_id.try_into()?));
        }

        if let Some(guild_id) = self.guild_id {
            selector.guild(GuildId::new(guild_id.try_into()?));
        }

        Ok(selector)
    }
}

pub async fn fetch_selectors_for_matcher(
    matcher_id: i64,
    pool: &SqlitePool,
) -> Result<Vec<SelectorModel>> {
    sqlx::query_as!(
        SelectorModel,
        "SELECT * FROM selector
        WHERE content_matcher_id = $1",
        matcher_id
    )
    .fetch_all(pool)
    .await
    .wrap_err("Failed to fetch responses for matcher.")
}

pub fn selectors_from_models(models: Vec<SelectorModel>) -> Result<Vec<Selector>> {
    let mut selectors: Vec<Selector> = Vec::new();

    for model in models {
        selectors.push(model.into_selector()?);
    }

    Ok(selectors)
}
