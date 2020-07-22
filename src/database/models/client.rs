use crate::scopes::Scopes;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub struct Client {
    /// The Id of the client
    pub id: i64,
    pub owner: String,
    pub name: String,
    pub display_name: String,
    pub icon_url: String,
    pub public_key: String,
    // This will be encrypted for security.
    pub secret_key: String,
    pub return_urls: Vec<String>,
    pub granted_scopes: Vec<Scopes>,
}

//region Utility
fn gen_string(size: usize) -> String
{
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .collect()
}
//endregion