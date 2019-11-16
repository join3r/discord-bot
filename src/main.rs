mod subs;
use subs::{MError, XkcdResponse, DiscordWebhook, Webs};

const DISCORD_WEBHOOK: &str = include_str!("./webhook.secret");

fn main() -> Result<(), MError> {
    let xkcd_new = XkcdResponse::get("https://xkcd.com/info.0.json")?;
    let xkcd_last = match XkcdResponse::load_from_file("./xkcd.last".into()) {
        Ok(x) => x,
        Err(_) => Default::default(),
    };
    if xkcd_last != xkcd_new {
        xkcd_new.save_to_file("./xkcd.last".into())?;
        DiscordWebhook::post(&xkcd_new.into(),DISCORD_WEBHOOK)?;
    };
    Ok(())
}