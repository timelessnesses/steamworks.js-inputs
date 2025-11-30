use std::sync::{Arc, Mutex};
use steamworks::Client;

lazy_static! {
    static ref STEAM_CLIENT: Mutex<Option<Arc<Client>>> = Mutex::new(None);
}

pub fn has_client() -> bool {
    STEAM_CLIENT.lock().unwrap().is_some()
}

pub fn get_client() -> Result<Arc<Client>, &'static str> {
    // println!("Getting Steam client... Current strong count: {}", STEAM_CLIENT.lock().unwrap().as_ref().map_or(0, |arc| Arc::strong_count(arc)));
    let option = STEAM_CLIENT.lock().unwrap().to_owned();
    option.ok_or_else(|| "Steam client's not initialized.")
}

pub fn set_client(client: Client) {
    let mut client_ref = STEAM_CLIENT.lock().unwrap();
    *client_ref = Some(Arc::new(client));
}

pub fn drop_client() {
    if !has_client() {
        return;
    }
    let mut client_ref = STEAM_CLIENT.lock().unwrap();
    if let Some(arc) = client_ref.take() {
        match Arc::try_unwrap(arc) {
            Ok(client) => {
                drop(client);
            },
            Err(arc) => {
                println!("Warning: Tried to drop Steam client but there are still {} strong references to it.", Arc::strong_count(&arc));
                *client_ref = Some(arc);
            }
        }
    }
}
