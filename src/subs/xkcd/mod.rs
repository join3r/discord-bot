use serde::{Deserialize, Serialize};
use super::Webs;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XkcdResponse {
  safe_title: String,
  alt: String,
  img: String,
  title: String,
}

impl Webs<XkcdResponse> for XkcdResponse {}

use super::discord::{DiscordWebhook, Embed, Image};

impl From<XkcdResponse> for DiscordWebhook {
  fn from(source: XkcdResponse) -> Self {
    DiscordWebhook {
      username: "xkcd".into(),
      content: format!("{}\n{}\n", source.title, source.alt),
      embeds: vec!(Embed {
        image: Image { url: source.img }
      }),
    }
  }
}