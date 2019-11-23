use serde::Serialize;
use super::Webs;

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DiscordWebhook {
    pub username: String,
    pub content: String,
    pub embeds: Vec<Embed>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Embed {
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Image {
    pub url: String,
}

impl Webs<DiscordWebhook> for DiscordWebhook {}