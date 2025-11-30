use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

#[napi(object)]
pub struct FriendInfo {
    pub name: String,
    pub nick_name: Option<String>,
    pub small_avatar: Option<Buffer>,
    pub medium_avatar: Option<Buffer>,
    pub large_avatar: Option<Buffer>,
}

impl From<steamworks::Friend> for FriendInfo {
    fn from(value: steamworks::Friend) -> Self {
        FriendInfo {
            name: value.name(),
            nick_name: value.nick_name(),
            small_avatar: value.small_avatar().map(|i| {
                Buffer::from(i)
            }),
            medium_avatar: value.medium_avatar().map(|i| {
                Buffer::from(i)
            }),
            large_avatar: value.large_avatar().map(|i| {
                Buffer::from(i)
            }),
        }
    }
}

#[napi]
pub mod friends {
    use super::FriendInfo;
    use napi::bindgen_prelude::BigInt;
    use tokio::sync::oneshot;
    use steamworks::PersonaStateChange;

    #[napi]
    pub async fn request_user_information(steam_id: BigInt, require_name_only: bool, timeout_seconds: Option<u32>) -> Result<FriendInfo, napi::Error> {
        let client = crate::client::get_client().unwrap();
        let steam_id = steamworks::SteamId::from_raw(steam_id.get_u64().1);
        if client
            .friends()
            .request_user_information(steam_id, require_name_only)
        {
            let (tx, rx) = oneshot::channel();
            let mut tx = Some(tx);
            println!("Creating callback for {}", steam_id.steamid32());

            let callback = client.register_callback(move |player: PersonaStateChange| {
                println!("Callback for {}", player.steam_id.steamid32()); // TODO: Remove
                if player.steam_id == steam_id {
                    println!("Sending persona state change for {}", steam_id.steamid32());
                    if let Some(tx) = tx.take() {
                        tx.send(player).unwrap();
                    }
                }
            });
            let timeout_seconds = timeout_seconds.unwrap_or(10) as u64;
            println!("Waiting for callback for {} for {} seconds", steam_id.steamid32(), timeout_seconds);
            let result = tokio::time::timeout(std::time::Duration::from_secs(timeout_seconds), rx).await;
            println!("Done waiting for callback for {}", steam_id.steamid32());
            drop(callback);
            println!("Result: {:?}", result);
            match result {
                Err(_) => {
                    return Err(napi::Error::from_reason("Steam didn't validated the ticket in time."))
                },
                Ok(Err(e)) => {
                    return Err(napi::Error::from_reason(format!("Failed to receive persona state change: {}", e)))
                },
                _ => {}

            }
        }
        println!("Fetched user information for {}", steam_id.steamid32());
        Ok(client.friends().get_friend(steam_id).into())
    }
}
