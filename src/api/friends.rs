use napi::bindgen_prelude::{BigInt, Buffer};
use napi_derive::napi;

#[napi(object)]
pub struct FriendInfo {
    pub name: String,
    pub nick_name: Option<String>,
    pub small_avatar: Option<Buffer>,
    pub medium_avatar: Option<Buffer>,
    pub large_avatar: Option<Buffer>,
    pub id: BigInt,
}

impl From<steamworks::Friend> for FriendInfo {
    fn from(value: steamworks::Friend) -> Self {
        FriendInfo {
            name: value.name(),
            nick_name: value.nick_name(),
            small_avatar: value.small_avatar().map(|i| Buffer::from(i)),
            medium_avatar: value.medium_avatar().map(|i| Buffer::from(i)),
            large_avatar: value.large_avatar().map(|i| Buffer::from(i)),
            id: BigInt::from(value.id().raw()),
        }
    }
}

#[napi]
pub mod friends {
    
    use std::ops::{Deref, DerefMut};

    use crate::api::friends::pretty_panic_but_not_panic;

    use super::FriendInfo;
    use napi::{Error, bindgen_prelude::BigInt};
    use steamworks::{CallbackHandle, PersonaStateChange};
    use tokio::sync::oneshot;

    struct BetterCallback(CallbackHandle, steamworks::SteamId);
    impl Drop for BetterCallback {
        fn drop(&mut self) {
        }
    }

    impl Deref for BetterCallback {
        type Target = CallbackHandle;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for BetterCallback {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    #[napi]
    pub async fn request_user_information(
        steam_id: BigInt,
        require_name_only: bool,
        timeout_seconds: Option<u32>,
    ) -> Result<FriendInfo, napi::Error> {
        let client = crate::client::get_client().map_err(|i| Error::from_reason(i))?;
        let steam_id = steamworks::SteamId::from_raw(steam_id.get_u64().1);
        let (tx, rx) = oneshot::channel();
        let mut tx = Some(tx);
        println!("Registering callback for {}", steam_id.steamid32());
        let callback = client.register_callback(move |player: PersonaStateChange| {
            println!("Callback for {}", player.steam_id.steamid32()); // TODO: Remove
            if player.steam_id == steam_id {
                println!("Sending persona state change for {}", steam_id.steamid32());
                if let Some(tx) = tx.take() {
                    match tx.send(player) {
                        Ok(_) => {
                            println!("Sent persona state change for {}", steam_id.steamid32())
                        }
                        Err(e) => println!(
                            "Failed to send persona state change for {}: {:#?}",
                            steam_id.steamid32(),
                            e
                        ),
                    }
                }
            }
        });
        let callback = BetterCallback(callback, steam_id);
        if client
            .friends()
            .request_user_information(steam_id, require_name_only)
        {
            client.run_callbacks();
            let timeout_seconds = timeout_seconds.unwrap_or(10) as u64;
            println!(
                "Waiting for callback for {} for {} seconds",
                steam_id.steamid32(),
                timeout_seconds
            );
            let result =
                tokio::time::timeout(std::time::Duration::from_secs(timeout_seconds), rx).await;
            println!("Done waiting for callback for {}", steam_id.steamid32());
            println!("Result for ID {}: {:?}", steam_id.steamid32(), result);
            // pretty_panic_but_not_panic("hiiii");
            drop(callback);
            match result {
                Err(_) => {
                    // panic!("timeout waiting for {}'s persona state change", steam_id.steamid32());
                    pretty_panic_but_not_panic(&format!(
                        "timeout waiting for {}'s persona state change",
                        steam_id.steamid32()
                    ));
                    return Err(napi::Error::from_reason(
                        "Steam did not callback in time".to_string(),
                    ));
                }
                Ok(Err(e)) => {
                    // panic!("oneshot receive error for {}: {}", steam_id.steamid32(), e);
                    pretty_panic_but_not_panic(&format!(
                        "oneshot receive error for {}: {}\nLet's do a loop request since steam's an asshole.",
                        steam_id.steamid32(),
                        e
                    ));
                    loop {
                        if !client.friends().request_user_information(steam_id, require_name_only) {
                            break;
                        }
                    }
                    println!("Fetched user information for {} after multiple rounds of abuse.", steam_id.steamid32());
                    return Ok(client.friends().get_friend(steam_id).into());
                }
                _ => {
                    println!("Fetched user information for {}, the steamcallback actually works!", steam_id.steamid32());
                    Ok(client.friends().get_friend(steam_id).into())
                }
            }
        }
        // drop(callback);
        // println!("Dropping callback for {}", steam_id.steamid32());
        // println!("Fetched user information for {}", steam_id.steamid32());
        else {
            println!("Fetched user information for {} without a callback (precached)", steam_id.steamid32());
            drop(callback);
            Ok(client.friends().get_friend(steam_id).into())
        }
    }
}

fn pretty_panic_but_not_panic(msg: &str) {
    let _ = std::panic::catch_unwind(|| panic!("{}", msg));
}
