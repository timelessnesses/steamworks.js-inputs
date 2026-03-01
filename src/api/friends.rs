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
    use super::FriendInfo;
    use napi::{bindgen_prelude::BigInt, Error};
    use steamworks::PersonaStateChange;
    use tokio::sync::oneshot;

    #[napi]
    pub async fn request_user_information(
        steam_id: BigInt,
        require_name_only: bool,
        timeout_seconds: Option<u32>,
    ) -> Result<FriendInfo, napi::Error> {
        let client = crate::client::get_client().map_err(|i| Error::from_reason(i))?;
        let steam_id = steamworks::SteamId::from_raw(steam_id.get_u64().1);
        
        // Check if information is already cached
        if !client
            .friends()
            .request_user_information(steam_id, require_name_only)
        {
            // Information is already cached, return immediately
            return Ok(client.friends().get_friend(steam_id).into());
        }

        // Information needs to be fetched, wait for callback
        let (tx, rx) = oneshot::channel();
        let mut tx = Some(tx);
        
        // Register callback - it will persist in global registry even if not stored
        let _callback = client.register_callback(move |player: PersonaStateChange| {
            if player.steam_id == steam_id {
                if let Some(tx) = tx.take() {
                    let _ = tx.send(());
                }
            }
        });

        let timeout_seconds = timeout_seconds.unwrap_or(10) as u64;
        let result =
            tokio::time::timeout(std::time::Duration::from_secs(timeout_seconds), rx).await;

        // Explicitly drop the callback handle to clean up
        drop(_callback);

        match result {
            Ok(Ok(())) => Ok(client.friends().get_friend(steam_id).into()),
            Ok(Err(_)) => Err(napi::Error::from_reason(
                "Internal error: callback channel closed unexpectedly".to_string(),
            )),
            Err(_) => Err(napi::Error::from_reason(
                "Timeout waiting for user information".to_string(),
            )),
        }
    }
}
