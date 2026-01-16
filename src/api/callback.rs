use napi_derive::napi;
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref CALLBACK_REGISTRY: Mutex<HashMap<u64, steamworks::CallbackHandle>> = Mutex::new(HashMap::new());
    static ref NEXT_ID: Mutex<u64> = Mutex::new(0);
}

#[napi]
pub mod callback {
    use napi::{
        bindgen_prelude::Function,
        threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    };

    #[napi]
    pub struct Handle {
        id: u64,
        disconnected: bool,
    }

    #[napi]
    impl Handle {
        #[napi]
        pub fn disconnect(&mut self) {
            if !self.disconnected {
                // Remove from global registry, which will drop the callback handle
                super::CALLBACK_REGISTRY.lock().unwrap().remove(&self.id);
                self.disconnected = true;
            }
        }
    }

    impl Drop for Handle {
        fn drop(&mut self) {
            // If handle is being dropped without explicit disconnect,
            // leave it in the global registry to prevent premature callback unregistration
            if !self.disconnected {
                println!("Warning: Callback handle {} is being dropped without explicit disconnect. Callback will remain active.", self.id);
            }
        }
    }

    #[napi]
    pub enum SteamCallback {
        PersonaStateChange,
        SteamServersConnected,
        SteamServersDisconnected,
        SteamServerConnectFailure,
        LobbyDataUpdate,
        LobbyChatUpdate,
        P2PSessionRequest,
        P2PSessionConnectFail,
        GameLobbyJoinRequested,
        MicroTxnAuthorizationResponse,
        LobbyChatMessage,
    }

    #[napi(ts_generic_types = "C extends keyof import('./callbacks').CallbackReturns")]
    pub fn register(
        #[napi(ts_arg_type = "C")] steam_callback: SteamCallback,
        #[napi(ts_arg_type = "(value: import('./callbacks').CallbackReturns[C]) => void")] handler: Function<'static>,
    ) -> Handle {
        let threadsafe_handler: ThreadsafeFunction<
            serde_json::Value,
            napi::Unknown<'_>,
            Vec<serde_json::Value>,
            napi::Status,
            false,
        > = handler
            .build_threadsafe_function::<serde_json::Value>()
            .callee_handled::<false>()
            .max_queue_size::<0>()
            .build_callback(|ctx| Ok(vec![ctx.value]))
            .unwrap();

        let handle = match steam_callback {
            SteamCallback::PersonaStateChange => {
                register_callback::<steamworks::PersonaStateChange>(threadsafe_handler)
            }
            SteamCallback::SteamServersConnected => {
                register_callback::<steamworks::SteamServersConnected>(threadsafe_handler)
            }
            SteamCallback::SteamServersDisconnected => {
                register_callback::<steamworks::SteamServersDisconnected>(threadsafe_handler)
            }
            SteamCallback::SteamServerConnectFailure => {
                register_callback::<steamworks::SteamServerConnectFailure>(threadsafe_handler)
            }
            SteamCallback::LobbyDataUpdate => {
                register_callback::<steamworks::LobbyDataUpdate>(threadsafe_handler)
            }
            SteamCallback::LobbyChatUpdate => {
                register_callback::<steamworks::LobbyChatUpdate>(threadsafe_handler)
            }
            SteamCallback::P2PSessionRequest => {
                register_callback::<steamworks::P2PSessionRequest>(threadsafe_handler)
            }
            SteamCallback::P2PSessionConnectFail => {
                register_callback::<steamworks::P2PSessionConnectFail>(threadsafe_handler)
            }
            SteamCallback::GameLobbyJoinRequested => {
                register_callback::<steamworks::GameLobbyJoinRequested>(threadsafe_handler)
            }
            SteamCallback::MicroTxnAuthorizationResponse => {
                register_callback::<steamworks::MicroTxnAuthorizationResponse>(threadsafe_handler)
            }
            SteamCallback::LobbyChatMessage => {
                register_callback::<steamworks::LobbyChatMsg>(threadsafe_handler)
            }
        };

        // Generate unique ID and store in global registry to prevent premature dropping
        let id = {
            let mut next_id = super::NEXT_ID.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };
        
        super::CALLBACK_REGISTRY.lock().unwrap().insert(id, handle);

        Handle {
            id,
            disconnected: false,
        }
    }

    fn register_callback<C>(
        threadsafe_handler: ThreadsafeFunction<
            serde_json::Value,
            napi::Unknown<'_>,
            Vec<serde_json::Value>,
            napi::Status,
            false,
        >,
    ) -> steamworks::CallbackHandle
    where
        C: steamworks::Callback + serde::Serialize,
    {
        let client = crate::client::get_client().unwrap();
        client.register_callback(move |value: C| {
            let value = serde_json::to_value(&value).unwrap();
            threadsafe_handler.call(value, ThreadsafeFunctionCallMode::Blocking);
        })
    }
}
