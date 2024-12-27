use crate::commands::{Context, Error, utils::*};

/// Modify the bot's responses.
#[poise::command(
    prefix_command,
    track_edits,
    slash_command,
    subcommands("add", "remove")
)]
pub async fn response(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Add a response.
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn add(
    ctx: Context<'_>,
    // TODO: Add autocomplete
    #[description = "TODO: Add description"] triggers: String,
    #[description = "TODO: Add description"] responses: String,
) -> Result<(), Error> {
    reply_error(ctx, "not implemented").await
}

/// Remove a response.
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "TODO: Add description"] id: String,
) -> Result<(), Error> {
    reply_error(ctx, "not implemented").await
}
