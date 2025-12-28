use napi_derive::napi;

#[napi]
pub mod utils {
    use std::ops::Deref;
    use std::ops::DerefMut;

    use steamworks::FloatingGamepadTextInputMode as kFloatingGamepadTextInputMode;
    use steamworks::GamepadTextInputLineMode as kGamepadTextInputLineMode;
    use steamworks::GamepadTextInputMode as kGamepadTextInputMode;
    use tokio::sync::oneshot;

    #[napi]
    pub fn get_app_id() -> u32 {
        let client = crate::client::get_client().unwrap();
        client.utils().app_id().0
    }

    #[napi]
    pub fn get_server_real_time() -> u32 {
        let client = crate::client::get_client().unwrap();
        client.utils().get_server_real_time()
    }

    #[napi]
    pub fn is_steam_running_on_steam_deck() -> bool {
        let client = crate::client::get_client().unwrap();
        client.utils().is_steam_running_on_steam_deck()
    }

    #[napi]
    pub enum GamepadTextInputMode {
        Normal,
        Password,
    }

    #[napi]
    pub enum GamepadTextInputLineMode {
        SingleLine,
        MultipleLines,
    }

    struct DropDetector<T>(T);

    impl<T> Deref for DropDetector<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> Drop for DropDetector<T> {
        fn drop(&mut self) {
            println!("DropDetector dropped");
        }
    }
    impl<T> DerefMut for DropDetector<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    /// @returns the entered text, or null if cancelled or could not show the input
    #[napi]
    pub async fn show_gamepad_text_input(
        input_mode: GamepadTextInputMode,
        input_line_mode: GamepadTextInputLineMode,
        description: String,
        max_characters: u32,
        existing_text: Option<String>,
    ) -> Option<String> {
        let client = crate::client::get_client().unwrap();

        let (tx, rx) = oneshot::channel();
        let mut tx = Some(tx);

        let (opened, _cb) = client.utils().show_gamepad_text_input(
            match input_mode {
                GamepadTextInputMode::Normal => kGamepadTextInputMode::Normal,
                GamepadTextInputMode::Password => kGamepadTextInputMode::Password,
            },
            match input_line_mode {
                GamepadTextInputLineMode::SingleLine => kGamepadTextInputLineMode::SingleLine,
                GamepadTextInputLineMode::MultipleLines => kGamepadTextInputLineMode::MultipleLines,
            },
            &description,
            max_characters,
            existing_text.as_deref(),
            move |dismissed_data| {
                println!("Gamepad text input dismissed");
                if let Some(tx) = tx.take() {
                    println!("Sending entered text back");
                    let text = client
                        .utils()
                        .get_entered_gamepad_text_input(&dismissed_data);
                    println!("Sending entered text back: {:#?}", text);
                    tx.send(text).unwrap();
                    println!("Sent entered text back");
                }
            },
        );
        println!("show_gamepad_text_input opened: {}", opened);
        if opened {
            let a = rx.await.ok().flatten();
            println!("show_gamepad_text_input result: {:#?}", a);
            a
        } else {
            None
        }
    }

    #[napi]
    pub enum FloatingGamepadTextInputMode {
        SingleLine,
        MultipleLines,
        Email,
        Numeric,
    }

    /// @returns true if the floating keyboard was shown, otherwise, false
    #[napi]
    pub async fn show_floating_gamepad_text_input(
        keyboard_mode: FloatingGamepadTextInputMode,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> bool {
        let client = crate::client::get_client().unwrap();

        let (tx, rx) = oneshot::channel();
        let mut tx = Some(tx);

        let (opened, _cb) = client.utils().show_floating_gamepad_text_input(
            match keyboard_mode {
                FloatingGamepadTextInputMode::SingleLine => {
                    kFloatingGamepadTextInputMode::SingleLine
                }
                FloatingGamepadTextInputMode::MultipleLines => {
                    kFloatingGamepadTextInputMode::MultipleLines
                }
                FloatingGamepadTextInputMode::Email => kFloatingGamepadTextInputMode::Email,
                FloatingGamepadTextInputMode::Numeric => kFloatingGamepadTextInputMode::Numeric,
            },
            x,
            y,
            width,
            height,
            move || {
                if let Some(tx) = tx.take() {
                    tx.send(true).unwrap();
                }
            },
        );

        if opened {
            rx.await.unwrap()
        } else {
            false
        }
    }
}
