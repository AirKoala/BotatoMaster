use crate::commands::{Context, Error, utils::*};

/// List all preferences.
#[poise::command(prefix_command, track_edits, aliases("settings", "options"), slash_command)]
pub async fn preferences(
    ctx: Context<'_>,
) -> Result<(), Error> {
    reply_error(ctx, "not implemented").await
}
