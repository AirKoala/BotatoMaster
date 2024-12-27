use regex::Regex;
use serenity::model::{
    channel::Message,
    id::{ChannelId, GuildId, UserId},
};

#[derive(Default, Debug)]
pub struct Selector {
    regex: Option<Regex>,
    author: Option<UserId>,
    channel: Option<ChannelId>,
    guild: Option<GuildId>,
}
impl Selector {
    pub fn regex(&mut self, regex: Regex) -> &Self {
        self.regex = Some(regex);
        self
    }

    pub fn regex_str(&mut self, regex: &str) -> Result<&Self, regex::Error> {
        Ok(self.regex(Regex::new(regex)?))
    }

    pub fn author(&mut self, author: UserId) -> &Self {
        self.author = Some(author);
        self
    }
    pub fn channel(&mut self, channel: ChannelId) -> &Self {
        self.channel = Some(channel);
        self
    }
    pub fn guild(&mut self, guild: GuildId) -> &Self {
        self.guild = Some(guild);
        self
    }

    pub fn matches(&self, message: &Message) -> bool {
        if let Some(re) = &self.regex {
            if !re.is_match(&message.content) {
                return false;
            }
        }

        if let Some(author) = self.author {
            if author != message.author.id {
                return false;
            }
        }

        if let Some(channel) = self.channel {
            if channel != message.channel_id {
                return false;
            }
        }

        if !self.guild.is_none() && self.guild != message.guild_id {
            return false;
        }

        true
    }
}
