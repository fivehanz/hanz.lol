use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Link {
    pub slug: String,
    pub href: String,
}

#[derive(Deserialize)]
pub struct LinkSlug {
    pub slug: String,
}
