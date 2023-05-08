// !written with immense help from codeium chat & chatGPT 3.5

extern crate wasm_bindgen;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use worker_kv::{KvError, KvStore};

const NAMESPACE_NAME: &str = "links";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Link {
    pub slug: String,
    pub href: String,
}

#[derive(Deserialize)]
pub struct LinkSlug {
    pub slug: String,
}

// Define an interface called `Kv`
#[async_trait]
pub trait Kv {
    fn get_key(&self) -> String;
    fn get_value(&self) -> String;
    fn from_value(value: String) -> Self
    where
        Self: Sized;
    fn store(&self) -> Result<(), JsValue>;
    // async fn retrieve(key: String) -> Result<Self, KvError>
    // where
    //     Self: Sized;
}

#[async_trait]
impl Kv for Link {
    fn get_key(&self) -> String {
        self.slug.clone()
    }

    fn get_value(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_value(value: String) -> Self {
        serde_json::from_str(&value).unwrap_or_default()
    }

    fn store(&self) -> Result<(), JsValue> {
        // Connect to the Cloudflare KV namespace
        let namespace = match KvStore::create(NAMESPACE_NAME) {
            Ok(namespace) => namespace,
            Err(e) => return Err(JsValue::from_str(&format!("Error: {:?}", e))),
        };

        // Store the link in the KV namespace
        let result = namespace
            .put(&self.get_key(), self.get_value().as_bytes())
            .map_err(|e| JsValue::from_str(&format!("Error: {:?}", e)))
            .map(|_| ());
        result
    }

    // async fn retrieve(key: String) -> Result<Self, KvError> {
    //     // Connect to the Cloudflare KV namespace
    //     let namespace = match KvStore::create("NAMESPACE_NAME") {
    //         Ok(namespace) => namespace,
    //         Err(e) => return Err(e),
    //     };

    //     // Spawn a new Tokio task to retrieve the value from the KV namespace
    //     let result = tokio::spawn(async move {
    //         let response = namespace.get(&key).text().await.unwrap_or_default();
    //         Link::from_value(response.unwrap())
    //     })
    //     .await;

    //     match result {
    //         Ok(link) => Ok(link),
    //         // TODO: Handle errors, !make it generic
    //         Err(e) => Err(e),
    //     }
    // }
}

pub fn set_link(slug: String, href: String) -> Result<(), JsValue> {
    let link = Link { slug, href };
    link.store()
}
