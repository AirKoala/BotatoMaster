use crate::commands::{Context, Error};
use std::fmt::Display;

pub async fn reply_error(ctx: Context<'_>, error: impl Display) -> Result<(), Error> {
    ctx.reply(format!("```{}```", error)).await?;
    Err(error.to_string().into())
}
