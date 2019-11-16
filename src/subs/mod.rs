pub mod discord;
pub mod xkcd;

pub use discord::DiscordWebhook;
pub use xkcd::XkcdResponse;

pub use failure::Error as MError;

pub trait Webs<T> {
    fn save_to_file(json: &T, filename: std::path::PathBuf) -> Result<&T, MError> 
    where T: serde::ser::Serialize {
        let file = std::fs::File::create(filename)?;
        serde_json::to_writer(file, json)?;
        Ok(json)
    }
    fn load_from_file(filename: std::path::PathBuf) -> Result<T, MError> 
    where T: serde::de::DeserializeOwned {
        let file = std::fs::File::open(filename)?;
        Ok(serde_json::from_reader(file)?)
    }
    fn get(url: &str) -> Result<T, MError> 
    where T: serde::de::DeserializeOwned {
        let body = reqwest::Client::new().get(url).send()?.text()?;
        Ok(serde_json::from_str(&body)?)
    }
    fn post(json: T, url: &str) -> Result<T, MError>
    where T: serde::ser::Serialize {
        reqwest::Client::new().post(url).json(&json).send()?;
        Ok(json)
    }
}