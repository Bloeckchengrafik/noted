use gpui::{AssetSource, SharedString};
use gpui::http_client::anyhow;
use anyhow::Result;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../assets"]
pub struct Assets;

impl AssetSource for Assets {
  fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
    Self::get(path).map(|f| Some(f.data)).ok_or_else(|| anyhow!("asset \"{}\" not found", path))
  }

  fn list(&self, path: &str) -> Result<Vec<SharedString>> {
    Ok(Self::iter().filter_map(|p| if p.starts_with(path) { Some(p.into()) } else { None }).collect())
  }
}
