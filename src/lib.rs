use napi::bindgen_prelude::Error;
use napi_derive::napi;
use steamworks::AppId;
use steamworks::Client;
use steamworks::SteamAPIInitError;

pub mod client;

#[macro_use]
extern crate lazy_static;

#[napi]
pub fn init(app_id: Option<u32>) -> Result<(), Error> {
    if client::has_client() {
        client::drop_client();
    }

    let steam_client = app_id
        .map(|app_id| Client::init_app(AppId(app_id)))
        .unwrap_or_else(Client::init)
        .map_err(|e| match e {
            SteamAPIInitError::FailedGeneric(msg)
            | SteamAPIInitError::NoSteamClient(msg)
            | SteamAPIInitError::VersionMismatch(msg) => Error::from_reason(msg),
        })?;

    steam_client.user_stats().request_user_stats(steam_client.user().steam_id().raw());

    client::set_client(steam_client);
    Ok(())
}

#[napi]
pub fn restart_app_if_necessary(app_id: u32) -> bool {
    steamworks::restart_app_if_necessary(AppId(app_id))
}

#[napi]
pub fn run_callbacks() {
    client::get_client().run_callbacks();
}

pub mod api;
