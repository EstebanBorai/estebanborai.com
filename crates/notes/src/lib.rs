//! Notes types and utility functions

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Icon {
    Docker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub description: String,
    pub icon: Icon,
    pub date: String,
    pub preview_image_url: String,
}
