use std::error::Error;

use wasm_bindgen::JsValue;

use crate::kv::kv::WorkersKv;
use crate::types::links::{Link, LinkSlug};

// create a new link
// TODO: implement creation of a new link
// ! write the logic of creating the link in utils files and import it
// pub async fn set_link(link: Link) -> Result<(), JsValue> {
// create a new WorkersKv instance
// let kv = WorkersKv { kv };

// create a key value pair
// kv.put_text(&link.slug, &link.href, 60).await
//     // return the payload for now
//     (StatusCode::CREATED, Json(payload))
// }

// read and return the link
// TODO: implement reading of the link and returning the 'href'
// pub async fn get_link(key: LinkSlug) -> Result<Link, Error> {
//     // ! write the logic of getting the link in utils files and import it
// }

// TODO: implement updating the link 'href' based on 'key'
// pub async fn update_link(link: LinkSlug) -> Result<(), Error> {
// }

// TODO: implement deleting the link based on 'key'
// pub async fn delete(key: LinkSlug) -> Result<(), Error> {
// }
