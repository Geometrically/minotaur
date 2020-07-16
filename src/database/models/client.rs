use crate::scopes::Scopes;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub struct Client {
    pub owner: String,
    pub client_pub: String,
    // This will be encrypted for security.
    pub client_private: String,
    pub client_picture_url: String,
    pub return_urls: Vec<String>,
    pub granted_scopes: Vec<Scopes>
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