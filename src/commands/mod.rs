mod help;
mod preferences;
mod response;
mod utils;

use std::{
    // fmt::{Debug, Display},
    marker::Send,
    sync::Arc,
};

use poise::Framework;
use std::time::Duration;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;

// TODO: Add actual state management.
type Context<'a> = poise::Context<'a, (), Error>;

pub fn get_framework() -> Framework<(), Error> {
    let commands = vec![
        help::help(),
        preferences::preferences(),
        response::response(),
    ];

    let options = poise::FrameworkOptions {
        commands,
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            // additional_prefixes: vec![
            //     poise::Prefix::Literal("hey bot,"),
            //     poise::Prefix::Literal("hey bot"),
            // ],
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        // on_error: |error| Box::pin(on_error(error)),
        on_error: |error| {
            Box::pin(async move {
                println!("Error: {:?}", error);
            })
        },
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },

        // Every command invocation must pass this check to continue execution
        // command_check: Some(|ctx| {
        //     Box::pin(async move {
        //         if ctx.author().id == 123456789 {
        //             return Ok(false);
        //         }
        //         Ok(true)
        //     })
        // }),

        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                Ok(())
            })
        },
        ..Default::default()
    };

    // TODO: Move this to a config file
    let testing_guilds = vec![
        593359698800017418, // Test server
        521998209136984066, // KoolKids
    ];

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Commands framework started for {}", _ready.user.name);

                // TODO: Check if in debug mode and set registration mode accordingly

                // poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                for gid in testing_guilds {
                    // Register commands in specified guilds. This propagates faster than global registration.

                    if let Err(error) = poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        gid.into(),
                    )
                    .await
                    {
                        println!("Error registering commands in guild {}: {:?}", gid, error);
                    }
                }

                // NOTE: This will return some state management mechanism passed into the function.
                Ok(())
            })
        })
        .options(options)
        .build();

    framework
}
