#[napi_derive::napi]
pub mod socket {
    use napi::bindgen_prelude::{BigInt, Buffer};
    use napi_derive::napi;
    use steamworks::{
        self, networking_sockets::NetConnection, networking_types::NetworkingIdentity,
    };

    #[napi]
    pub const UNRELIABLE: i32 = steamworks::networking_types::SendFlags::UNRELIABLE.bits();
    #[napi]
    pub const NO_NAGLE: i32 = steamworks::networking_types::SendFlags::NO_NAGLE.bits();
    #[napi]
    pub const UNRELIABLE_NO_NAGLE: i32 =
        steamworks::networking_types::SendFlags::UNRELIABLE_NO_NAGLE.bits();
    #[napi]
    pub const NO_DELAY: i32 = steamworks::networking_types::SendFlags::NO_DELAY.bits();
    #[napi]
    pub const UNRELIABLE_NO_DELAY: i32 =
        steamworks::networking_types::SendFlags::UNRELIABLE_NO_DELAY.bits();
    #[napi]
    pub const RELIABLE: i32 = steamworks::networking_types::SendFlags::RELIABLE.bits();
    #[napi]
    pub const RELIABLE_NO_NAGLE: i32 =
        steamworks::networking_types::SendFlags::RELIABLE_NO_NAGLE.bits();
    #[napi]
    pub const USE_CURRENT_THREAD: i32 =
        steamworks::networking_types::SendFlags::USE_CURRENT_THREAD.bits();
    #[napi]
    pub const AUTO_RESTART_BROKEN_SESSION: i32 =
        steamworks::networking_types::SendFlags::AUTO_RESTART_BROKEN_SESSION.bits();

    #[napi]
    pub struct Socket {
        connection: NetConnection,
    }

    #[napi]
    pub fn connect_p2p(host: BigInt, port: i32) -> Result<Socket, napi::Error> {
        let client = crate::client::get_client().unwrap();
        client
            .networking_sockets()
            .connect_p2p(
                NetworkingIdentity::new_steam_id(steamworks::SteamId::from_raw(host.get_u64().1)),
                port,
                vec![],
            )
            .map_err(|_| napi::Error::from_reason("Failed to connect..."))
            .map(|connection| Socket { connection })
    }

    #[napi]
    impl Socket {
        #[napi]
        pub fn send_message(&self, data: &[u8], send_flags: i32) -> Result<(), napi::Error> {
            match self.connection.send_message(data, steamworks::networking_types::SendFlags::from_bits_truncate(send_flags)) {
                Ok(_) => { Ok(()) }
                Err(e) => {
                    match e {
                        steamworks::SteamError::InvalidParameter => {
                            Err(napi::Error::from_reason("Message is too big"))
                        },
                        steamworks::SteamError::InvalidState => {
                            Err(napi::Error::from_reason("Connection is not in a valid state to send messages"))
                        },
                        steamworks::SteamError::NoConnection => {
                            Err(napi::Error::from_reason("No connection exists"))
                        },
                        steamworks::SteamError::Ignored => {
                            Err(napi::Error::from_reason("NO_DELAY flag was set and the message was dropped because we were not ready to send it"))
                        },
                        steamworks::SteamError::LimitExceeded => {
                            Err(napi::Error::from_reason("Too much data was in queue to be sent"))
                        }
                        _ => {
                            Err(napi::Error::from_reason("An unknown error occurred while sending message"))
                        }
                    }
                }
            }
        }

        /// Please poll this method regularly to receive messages.
        #[napi]
        pub fn receive_message(&mut self, max_message: u32) -> Result<Vec<Buffer>, napi::Error> {
            let mut buffers = Vec::new();
            for a in self.connection.receive_messages(max_message as usize).map_err(|_| {
                napi::Error::from_reason("Invalid handle")
            })? {
                buffers.push(a.data().into());
            }
            Ok(buffers)
        }

        
    }
}
