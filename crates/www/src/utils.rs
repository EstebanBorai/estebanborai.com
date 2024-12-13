use anyhow::{bail, Context, Result};
use reqwest::Url;
use web_sys::window;

pub fn hostname() -> Result<Url> {
    if let Some(win) = window() {
        let origin_str = win
            .location()
            .origin()
            .map_err(|_| anyhow::anyhow!("Failed to retrieve Origin from Location"))?;

        return Url::parse(&origin_str).context("Failed to parse Origin into URL");
    }

    bail!("Failed to get Window from context")
}
