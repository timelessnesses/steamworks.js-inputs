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

pub mod friends {
    use super::FriendInfo;
    use napi::bindgen_prelude::BigInt;
    use napi_derive::napi;
    use std::sync::{Arc, Condvar, Mutex};
    use steamworks::PersonaStateChange;

    #[napi]
    pub fn request_user_information(steam_id: BigInt, require_name_only: bool) -> FriendInfo {
        let client = crate::client::get_client();
        let steam_id = steamworks::SteamId::from_raw(steam_id.get_u64().1);
        if client
            .friends()
            .request_user_information(steam_id, require_name_only)
        {
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            let pair2 = pair.clone();
            let callback = client.register_callback(move |player: PersonaStateChange| {
                if player.steam_id == steam_id {
                    let (lock, cvar) = &*pair2;
                    let mut started = lock.lock().unwrap();
                    *started = true;
                    cvar.notify_one();
                }
            });
            client.run_callbacks();
            let (lock, cvar) = &*pair;
            let started = lock.lock().unwrap();
            drop(cvar.wait(started).unwrap());
            drop(callback);
        }
        client.friends().get_friend(steam_id).into()
    }
}
