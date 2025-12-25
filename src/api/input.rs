use napi_derive::napi;

#[napi]
pub mod input {
    use std::ffi::CString;

    use napi::bindgen_prelude::{BigInt, Function};
    use steamworks::FloatingGamepadTextInputMode;

    #[napi(string_enum)]
    pub enum InputType {
        Unknown,
        SteamController,
        XBox360Controller,
        XBoxOneController,
        GenericGamepad,
        PS4Controller,
        AppleMFiController,
        AndroidController,
        SwitchJoyConPair,
        SwitchJoyConSingle,
        SwitchProController,
        MobileTouch,
        PS3Controller,
        PS5Controller,
        SteamDeckController,
    }

    impl From<steamworks::InputType> for InputType {
        fn from(input_type: steamworks::InputType) -> InputType {
            match input_type {
                steamworks::InputType::Unknown => InputType::Unknown,
                steamworks::InputType::SteamController => InputType::SteamController,
                steamworks::InputType::XBox360Controller => InputType::XBox360Controller,
                steamworks::InputType::XBoxOneController => InputType::XBoxOneController,
                steamworks::InputType::GenericGamepad => InputType::GenericGamepad,
                steamworks::InputType::PS4Controller => InputType::PS4Controller,
                steamworks::InputType::AppleMFiController => InputType::AppleMFiController,
                steamworks::InputType::AndroidController => InputType::AndroidController,
                steamworks::InputType::SwitchJoyConPair => InputType::SwitchJoyConPair,
                steamworks::InputType::SwitchJoyConSingle => InputType::SwitchJoyConSingle,
                steamworks::InputType::SwitchProController => InputType::SwitchProController,
                steamworks::InputType::MobileTouch => InputType::MobileTouch,
                steamworks::InputType::PS3Controller => InputType::PS3Controller,
                steamworks::InputType::PS5Controller => InputType::PS5Controller,
                steamworks::InputType::SteamDeckController => InputType::SteamDeckController,
            }
        }
    }

    #[napi]
    pub fn get_action_handle(action_name: String) -> BigInt {
        let client = crate::client::get_client().unwrap();
        BigInt::from(client.input().get_action_set_handle(&action_name))
    }

    #[napi]
    pub fn get_file_path_for_action(action_handle: InputActionOrigins) -> String {
        let client = crate::client::get_client().unwrap();
        client
            .input()
            .get_glyph_for_action_origin(action_handle.into())
    }

    #[napi]
    pub fn run_frame() {
        let client = crate::client::get_client().unwrap();
        client.input().run_frame();
    }

    #[napi]
    pub struct Controller {
        pub(crate) handle: BigInt,
    }

    #[napi]
    impl Controller {
        #[napi]
        pub fn activate_action_set(&self, action_set_handle: BigInt) -> bool {
            let client = crate::client::get_client().unwrap();
            client
                .input()
                .activate_action_set_handle(self.handle.get_u64().1, action_set_handle.get_u64().1);
            self.get_current_active_action_set().get_u64().1 == action_set_handle.get_u64().1
        }

        #[napi]
        pub fn is_digital_action_pressed(&self, action_handle: BigInt) -> bool {
            let client = crate::client::get_client().unwrap();
            client
                .input()
                .get_digital_action_data(self.handle.get_u64().1, action_handle.get_u64().1)
                .bState
        }

        #[napi]
        pub fn get_analog_action_vector(&self, action_handle: BigInt) -> AnalogActionVector {
            let client = crate::client::get_client().unwrap();
            let data = client
                .input()
                .get_analog_action_data(self.handle.get_u64().1, action_handle.get_u64().1);
            AnalogActionVector {
                x: data.x as f64,
                y: data.y as f64,
            }
        }

        #[napi]
        pub fn get_type(&self) -> InputType {
            let client = crate::client::get_client().unwrap();
            client
                .input()
                .get_input_type_for_handle(self.handle.get_u64().1)
                .into()
        }

        #[napi]
        pub fn get_handle(&self) -> BigInt {
            self.handle.clone()
        }

        /// Gets controller's motion sensors
        #[napi]
        pub fn get_motion_data(&self) -> MotionData {
            let client = crate::client::get_client().unwrap();
            return client
                .input()
                .get_motion_data(self.handle.get_u64().1)
                .into();
        }

        /// Triggers a vibration event
        /// It has intensity from 0 (off) to 65535 (max)
        /// use something like `setTimeout` to make a timed vibration
        #[napi]
        pub fn trigger_vibration(
            &self,
            left_speed_micro_second: u16,
            right_speed_micro_second: u16,
        ) {
            unsafe {
                let x = steamworks::sys::SteamAPI_SteamInput_v006();
                steamworks::sys::SteamAPI_ISteamInput_TriggerVibration(
                    x,
                    self.handle.get_u64().1,
                    left_speed_micro_second,
                    right_speed_micro_second,
                );
            }
        }

        #[napi]
        pub fn get_analog_action_origins(
            &self,
            action_set_handle: BigInt,
            analog_action_handle: BigInt,
        ) -> Vec<InputActionOrigins> {
            let client = crate::client::get_client().unwrap();
            let out = client.input().get_analog_action_origins(
                self.handle.get_u64().1,
                action_set_handle.get_u64().1,
                analog_action_handle.get_u64().1,
            );
            out.iter().map(|i| InputActionOrigins::from(*i)).collect()
        }

        #[napi]
        pub fn get_digital_action_origins(
            &self,
            action_set_handle: BigInt,
            digital_action_handle: BigInt,
        ) -> Vec<InputActionOrigins> {
            let client = crate::client::get_client().unwrap();
            let out = client.input().get_digital_action_origins(
                self.handle.get_u64().1,
                action_set_handle.get_u64().1,
                digital_action_handle.get_u64().1,
            );
            out.iter().map(|i| InputActionOrigins::from(*i)).collect()
        }

        #[napi]
        pub fn get_current_active_action_set(&self) -> BigInt {
            unsafe {
                let x = steamworks::sys::SteamAPI_SteamInput_v006();
                steamworks::sys::SteamAPI_ISteamInput_GetCurrentActionSet(
                    x,
                    self.handle.get_u64().1,
                )
                .into()
            }
        }
    }

    #[napi]
    pub enum VibrateSide {
        Left,
        Right,
    }

    /// The conversion will return 32767 if the value is not found
    #[napi]
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum InputActionOrigins {
        None = 0,
        SteamControllerA = 1,
        SteamControllerB = 2,
        SteamControllerX = 3,
        SteamControllerY = 4,
        SteamControllerLeftBumper = 5,
        SteamControllerRightBumper = 6,
        SteamControllerLeftGrip = 7,
        SteamControllerRightGrip = 8,
        SteamControllerStart = 9,
        SteamControllerBack = 10,
        SteamControllerLeftPadTouch = 11,
        SteamControllerLeftPadSwipe = 12,
        SteamControllerLeftPadClick = 13,
        SteamControllerLeftPadDPadNorth = 14,
        SteamControllerLeftPadDPadSouth = 15,
        SteamControllerLeftPadDPadWest = 16,
        SteamControllerLeftPadDPadEast = 17,
        SteamControllerRightPadTouch = 18,
        SteamControllerRightPadSwipe = 19,
        SteamControllerRightPadClick = 20,
        SteamControllerRightPadDPadNorth = 21,
        SteamControllerRightPadDPadSouth = 22,
        SteamControllerRightPadDPadWest = 23,
        SteamControllerRightPadDPadEast = 24,
        SteamControllerLeftTriggerPull = 25,
        SteamControllerLeftTriggerClick = 26,
        SteamControllerRightTriggerPull = 27,
        SteamControllerRightTriggerClick = 28,
        SteamControllerLeftStickMove = 29,
        SteamControllerLeftStickClick = 30,
        SteamControllerLeftStickDPadNorth = 31,
        SteamControllerLeftStickDPadSouth = 32,
        SteamControllerLeftStickDPadWest = 33,
        SteamControllerLeftStickDPadEast = 34,
        SteamControllerGyroMove = 35,
        SteamControllerGyroPitch = 36,
        SteamControllerGyroYaw = 37,
        SteamControllerGyroRoll = 38,
        SteamControllerReserved0 = 39,
        SteamControllerReserved1 = 40,
        SteamControllerReserved2 = 41,
        SteamControllerReserved3 = 42,
        SteamControllerReserved4 = 43,
        SteamControllerReserved5 = 44,
        SteamControllerReserved6 = 45,
        SteamControllerReserved7 = 46,
        SteamControllerReserved8 = 47,
        SteamControllerReserved9 = 48,
        SteamControllerReserved10 = 49,
        PS4X = 50,
        PS4Circle = 51,
        PS4Triangle = 52,
        PS4Square = 53,
        PS4LeftBumper = 54,
        PS4RightBumper = 55,
        PS4Options = 56,
        PS4Share = 57,
        PS4LeftPadTouch = 58,
        PS4LeftPadSwipe = 59,
        PS4LeftPadClick = 60,
        PS4LeftPadDPadNorth = 61,
        PS4LeftPadDPadSouth = 62,
        PS4LeftPadDPadWest = 63,
        PS4LeftPadDPadEast = 64,
        PS4RightPadTouch = 65,
        PS4RightPadSwipe = 66,
        PS4RightPadClick = 67,
        PS4RightPadDPadNorth = 68,
        PS4RightPadDPadSouth = 69,
        PS4RightPadDPadWest = 70,
        PS4RightPadDPadEast = 71,
        PS4CenterPadTouch = 72,
        PS4CenterPadSwipe = 73,
        PS4CenterPadClick = 74,
        PS4CenterPadDPadNorth = 75,
        PS4CenterPadDPadSouth = 76,
        PS4CenterPadDPadWest = 77,
        PS4CenterPadDPadEast = 78,
        PS4LeftTriggerPull = 79,
        PS4LeftTriggerClick = 80,
        PS4RightTriggerPull = 81,
        PS4RightTriggerClick = 82,
        PS4LeftStickMove = 83,
        PS4LeftStickClick = 84,
        PS4LeftStickDPadNorth = 85,
        PS4LeftStickDPadSouth = 86,
        PS4LeftStickDPadWest = 87,
        PS4LeftStickDPadEast = 88,
        PS4RightStickMove = 89,
        PS4RightStickClick = 90,
        PS4RightStickDPadNorth = 91,
        PS4RightStickDPadSouth = 92,
        PS4RightStickDPadWest = 93,
        PS4RightStickDPadEast = 94,
        PS4DPadNorth = 95,
        PS4DPadSouth = 96,
        PS4DPadWest = 97,
        PS4DPadEast = 98,
        PS4GyroMove = 99,
        PS4GyroPitch = 100,
        PS4GyroYaw = 101,
        PS4GyroRoll = 102,
        PS4DPadMove = 103,
        PS4Reserved1 = 104,
        PS4Reserved2 = 105,
        PS4Reserved3 = 106,
        PS4Reserved4 = 107,
        PS4Reserved5 = 108,
        PS4Reserved6 = 109,
        PS4Reserved7 = 110,
        PS4Reserved8 = 111,
        PS4Reserved9 = 112,
        PS4Reserved10 = 113,
        XBoxOneA = 114,
        XBoxOneB = 115,
        XBoxOneX = 116,
        XBoxOneY = 117,
        XBoxOneLeftBumper = 118,
        XBoxOneRightBumper = 119,
        XBoxOneMenu = 120,
        XBoxOneView = 121,
        XBoxOneLeftTriggerPull = 122,
        XBoxOneLeftTriggerClick = 123,
        XBoxOneRightTriggerPull = 124,
        XBoxOneRightTriggerClick = 125,
        XBoxOneLeftStickMove = 126,
        XBoxOneLeftStickClick = 127,
        XBoxOneLeftStickDPadNorth = 128,
        XBoxOneLeftStickDPadSouth = 129,
        XBoxOneLeftStickDPadWest = 130,
        XBoxOneLeftStickDPadEast = 131,
        XBoxOneRightStickMove = 132,
        XBoxOneRightStickClick = 133,
        XBoxOneRightStickDPadNorth = 134,
        XBoxOneRightStickDPadSouth = 135,
        XBoxOneRightStickDPadWest = 136,
        XBoxOneRightStickDPadEast = 137,
        XBoxOneDPadNorth = 138,
        XBoxOneDPadSouth = 139,
        XBoxOneDPadWest = 140,
        XBoxOneDPadEast = 141,
        XBoxOneDPadMove = 142,
        XBoxOneLeftGripLower = 143,
        XBoxOneLeftGripUpper = 144,
        XBoxOneRightGripLower = 145,
        XBoxOneRightGripUpper = 146,
        XBoxOneShare = 147,
        XBoxOneReserved6 = 148,
        XBoxOneReserved7 = 149,
        XBoxOneReserved8 = 150,
        XBoxOneReserved9 = 151,
        XBoxOneReserved10 = 152,
        XBox360A = 153,
        XBox360B = 154,
        XBox360X = 155,
        XBox360Y = 156,
        XBox360LeftBumper = 157,
        XBox360RightBumper = 158,
        XBox360Start = 159,
        XBox360Back = 160,
        XBox360LeftTriggerPull = 161,
        XBox360LeftTriggerClick = 162,
        XBox360RightTriggerPull = 163,
        XBox360RightTriggerClick = 164,
        XBox360LeftStickMove = 165,
        XBox360LeftStickClick = 166,
        XBox360LeftStickDPadNorth = 167,
        XBox360LeftStickDPadSouth = 168,
        XBox360LeftStickDPadWest = 169,
        XBox360LeftStickDPadEast = 170,
        XBox360RightStickMove = 171,
        XBox360RightStickClick = 172,
        XBox360RightStickDPadNorth = 173,
        XBox360RightStickDPadSouth = 174,
        XBox360RightStickDPadWest = 175,
        XBox360RightStickDPadEast = 176,
        XBox360DPadNorth = 177,
        XBox360DPadSouth = 178,
        XBox360DPadWest = 179,
        XBox360DPadEast = 180,
        XBox360DPadMove = 181,
        XBox360Reserved1 = 182,
        XBox360Reserved2 = 183,
        XBox360Reserved3 = 184,
        XBox360Reserved4 = 185,
        XBox360Reserved5 = 186,
        XBox360Reserved6 = 187,
        XBox360Reserved7 = 188,
        XBox360Reserved8 = 189,
        XBox360Reserved9 = 190,
        XBox360Reserved10 = 191,
        SwitchA = 192,
        SwitchB = 193,
        SwitchX = 194,
        SwitchY = 195,
        SwitchLeftBumper = 196,
        SwitchRightBumper = 197,
        SwitchPlus = 198,
        SwitchMinus = 199,
        SwitchCapture = 200,
        SwitchLeftTriggerPull = 201,
        SwitchLeftTriggerClick = 202,
        SwitchRightTriggerPull = 203,
        SwitchRightTriggerClick = 204,
        SwitchLeftStickMove = 205,
        SwitchLeftStickClick = 206,
        SwitchLeftStickDPadNorth = 207,
        SwitchLeftStickDPadSouth = 208,
        SwitchLeftStickDPadWest = 209,
        SwitchLeftStickDPadEast = 210,
        SwitchRightStickMove = 211,
        SwitchRightStickClick = 212,
        SwitchRightStickDPadNorth = 213,
        SwitchRightStickDPadSouth = 214,
        SwitchRightStickDPadWest = 215,
        SwitchRightStickDPadEast = 216,
        SwitchDPadNorth = 217,
        SwitchDPadSouth = 218,
        SwitchDPadWest = 219,
        SwitchDPadEast = 220,
        SwitchProGyroMove = 221,
        SwitchProGyroPitch = 222,
        SwitchProGyroYaw = 223,
        SwitchProGyroRoll = 224,
        SwitchDPadMove = 225,
        SwitchReserved1 = 226,
        SwitchReserved2 = 227,
        SwitchReserved3 = 228,
        SwitchReserved4 = 229,
        SwitchReserved5 = 230,
        SwitchReserved6 = 231,
        SwitchReserved7 = 232,
        SwitchReserved8 = 233,
        SwitchReserved9 = 234,
        SwitchReserved10 = 235,
        SwitchRightGyroMove = 236,
        SwitchRightGyroPitch = 237,
        SwitchRightGyroYaw = 238,
        SwitchRightGyroRoll = 239,
        SwitchLeftGyroMove = 240,
        SwitchLeftGyroPitch = 241,
        SwitchLeftGyroYaw = 242,
        SwitchLeftGyroRoll = 243,
        SwitchLeftGripLower = 244,
        SwitchLeftGripUpper = 245,
        SwitchRightGripLower = 246,
        SwitchRightGripUpper = 247,
        SwitchJoyConButtonN = 248,
        SwitchJoyConButtonE = 249,
        SwitchJoyConButtonS = 250,
        SwitchJoyConButtonW = 251,
        SwitchReserved15 = 252,
        SwitchReserved16 = 253,
        SwitchReserved17 = 254,
        SwitchReserved18 = 255,
        SwitchReserved19 = 256,
        SwitchReserved20 = 257,
        PS5X = 258,
        PS5Circle = 259,
        PS5Triangle = 260,
        PS5Square = 261,
        PS5LeftBumper = 262,
        PS5RightBumper = 263,
        PS5Option = 264,
        PS5Create = 265,
        PS5Mute = 266,
        PS5LeftPadTouch = 267,
        PS5LeftPadSwipe = 268,
        PS5LeftPadClick = 269,
        PS5LeftPadDPadNorth = 270,
        PS5LeftPadDPadSouth = 271,
        PS5LeftPadDPadWest = 272,
        PS5LeftPadDPadEast = 273,
        PS5RightPadTouch = 274,
        PS5RightPadSwipe = 275,
        PS5RightPadClick = 276,
        PS5RightPadDPadNorth = 277,
        PS5RightPadDPadSouth = 278,
        PS5RightPadDPadWest = 279,
        PS5RightPadDPadEast = 280,
        PS5CenterPadTouch = 281,
        PS5CenterPadSwipe = 282,
        PS5CenterPadClick = 283,
        PS5CenterPadDPadNorth = 284,
        PS5CenterPadDPadSouth = 285,
        PS5CenterPadDPadWest = 286,
        PS5CenterPadDPadEast = 287,
        PS5LeftTriggerPull = 288,
        PS5LeftTriggerClick = 289,
        PS5RightTriggerPull = 290,
        PS5RightTriggerClick = 291,
        PS5LeftStickMove = 292,
        PS5LeftStickClick = 293,
        PS5LeftStickDPadNorth = 294,
        PS5LeftStickDPadSouth = 295,
        PS5LeftStickDPadWest = 296,
        PS5LeftStickDPadEast = 297,
        PS5RightStickMove = 298,
        PS5RightStickClick = 299,
        PS5RightStickDPadNorth = 300,
        PS5RightStickDPadSouth = 301,
        PS5RightStickDPadWest = 302,
        PS5RightStickDPadEast = 303,
        PS5DPadNorth = 304,
        PS5DPadSouth = 305,
        PS5DPadWest = 306,
        PS5DPadEast = 307,
        PS5GyroMove = 308,
        PS5GyroPitch = 309,
        PS5GyroYaw = 310,
        PS5GyroRoll = 311,
        PS5DPadMove = 312,
        PS5LeftGrip = 313,
        PS5RightGrip = 314,
        PS5LeftFn = 315,
        PS5RightFn = 316,
        PS5Reserved5 = 317,
        PS5Reserved6 = 318,
        PS5Reserved7 = 319,
        PS5Reserved8 = 320,
        PS5Reserved9 = 321,
        PS5Reserved10 = 322,
        PS5Reserved11 = 323,
        PS5Reserved12 = 324,
        PS5Reserved13 = 325,
        PS5Reserved14 = 326,
        PS5Reserved15 = 327,
        PS5Reserved16 = 328,
        PS5Reserved17 = 329,
        PS5Reserved18 = 330,
        PS5Reserved19 = 331,
        PS5Reserved20 = 332,
        SteamDeckA = 333,
        SteamDeckB = 334,
        SteamDeckX = 335,
        SteamDeckY = 336,
        SteamDeckL1 = 337,
        SteamDeckR1 = 338,
        SteamDeckMenu = 339,
        SteamDeckView = 340,
        SteamDeckLeftPadTouch = 341,
        SteamDeckLeftPadSwipe = 342,
        SteamDeckLeftPadClick = 343,
        SteamDeckLeftPadDPadNorth = 344,
        SteamDeckLeftPadDPadSouth = 345,
        SteamDeckLeftPadDPadWest = 346,
        SteamDeckLeftPadDPadEast = 347,
        SteamDeckRightPadTouch = 348,
        SteamDeckRightPadSwipe = 349,
        SteamDeckRightPadClick = 350,
        SteamDeckRightPadDPadNorth = 351,
        SteamDeckRightPadDPadSouth = 352,
        SteamDeckRightPadDPadWest = 353,
        SteamDeckRightPadDPadEast = 354,
        SteamDeckL2SoftPull = 355,
        SteamDeckL2 = 356,
        SteamDeckR2SoftPull = 357,
        SteamDeckR2 = 358,
        SteamDeckLeftStickMove = 359,
        SteamDeckL3 = 360,
        SteamDeckLeftStickDPadNorth = 361,
        SteamDeckLeftStickDPadSouth = 362,
        SteamDeckLeftStickDPadWest = 363,
        SteamDeckLeftStickDPadEast = 364,
        SteamDeckLeftStickTouch = 365,
        SteamDeckRightStickMove = 366,
        SteamDeckR3 = 367,
        SteamDeckRightStickDPadNorth = 368,
        SteamDeckRightStickDPadSouth = 369,
        SteamDeckRightStickDPadWest = 370,
        SteamDeckRightStickDPadEast = 371,
        SteamDeckRightStickTouch = 372,
        SteamDeckL4 = 373,
        SteamDeckR4 = 374,
        SteamDeckL5 = 375,
        SteamDeckR5 = 376,
        SteamDeckDPadMove = 377,
        SteamDeckDPadNorth = 378,
        SteamDeckDPadSouth = 379,
        SteamDeckDPadWest = 380,
        SteamDeckDPadEast = 381,
        SteamDeckGyroMove = 382,
        SteamDeckGyroPitch = 383,
        SteamDeckGyroYaw = 384,
        SteamDeckGyroRoll = 385,
        SteamDeckReserved1 = 386,
        SteamDeckReserved2 = 387,
        SteamDeckReserved3 = 388,
        SteamDeckReserved4 = 389,
        SteamDeckReserved5 = 390,
        SteamDeckReserved6 = 391,
        SteamDeckReserved7 = 392,
        SteamDeckReserved8 = 393,
        SteamDeckReserved9 = 394,
        SteamDeckReserved10 = 395,
        SteamDeckReserved11 = 396,
        SteamDeckReserved12 = 397,
        SteamDeckReserved13 = 398,
        SteamDeckReserved14 = 399,
        SteamDeckReserved15 = 400,
        SteamDeckReserved16 = 401,
        SteamDeckReserved17 = 402,
        SteamDeckReserved18 = 403,
        SteamDeckReserved19 = 404,
        SteamDeckReserved20 = 405,
        HoripadM1 = 406,
        HoripadM2 = 407,
        HoripadL4 = 408,
        HoripadR4 = 409,
        Count = 410,
        MaximumPossibleValue = 32767,
    }

    impl From<steamworks::sys::EInputActionOrigin> for InputActionOrigins {
        fn from(input_action_origin: steamworks::sys::EInputActionOrigin) -> InputActionOrigins {
            debug_assert_eq!(
                std::mem::size_of::<steamworks::sys::EInputActionOrigin>(),
                std::mem::size_of::<InputActionOrigins>()
            );
            unsafe {
                if (0..=409).contains(&(input_action_origin as usize)) {
                    return std::mem::transmute::<i32, InputActionOrigins>(
                        input_action_origin as i32,
                    );
                } else {
                    return InputActionOrigins::Count;
                }
            }
        }
    }

    impl Into<steamworks::sys::EInputActionOrigin> for InputActionOrigins {
        fn into(self) -> steamworks::sys::EInputActionOrigin {
            debug_assert_eq!(
                std::mem::size_of::<steamworks::sys::EInputActionOrigin>(),
                std::mem::size_of::<InputActionOrigins>()
            );
            unsafe {
                if (0..=409).contains(&(self as usize)) {
                    return std::mem::transmute::<
                        InputActionOrigins,
                        steamworks::sys::EInputActionOrigin,
                    >(self);
                } else {
                    return steamworks::sys::EInputActionOrigin::k_EInputActionOrigin_Count;
                }
            }
        }
    }

    #[napi(object)]
    pub struct MotionData {
        /// Absolute Rotation (drift) X axis
        pub rot_quat_x: f64,
        /// Absolute Rotation (drift) Y axis
        pub rot_quat_y: f64,
        /// Absolute Rotation (drift) Z axis
        pub rot_quat_z: f64,
        /// Absolute Rotation (drift) W axis
        pub rot_quat_w: f64,
        /// Positional Acceleration X axis
        pub pos_accel_x: f64,
        /// Positional Acceleration Y axis
        pub pos_accel_y: f64,
        /// Positional Acceleration Z axis
        pub pos_accel_z: f64,
        /// Rotational Velocity X axis
        pub rot_vel_x: f64,
        /// Rotational Velocity Y axis
        pub rot_vel_y: f64,
        /// Rotational Velocity Z axis
        pub rot_vel_z: f64,
    }

    impl From<steamworks::sys::InputMotionData_t> for MotionData {
        fn from(motion_data: steamworks::sys::InputMotionData_t) -> MotionData {
            MotionData {
                rot_quat_x: motion_data.rotQuatX as f64,
                rot_quat_y: motion_data.rotQuatY as f64,
                rot_quat_z: motion_data.rotQuatZ as f64,
                rot_quat_w: motion_data.rotQuatW as f64,
                pos_accel_x: motion_data.posAccelX as f64,
                pos_accel_y: motion_data.posAccelY as f64,
                pos_accel_z: motion_data.posAccelZ as f64,
                rot_vel_x: motion_data.rotVelX as f64,
                rot_vel_y: motion_data.rotVelY as f64,
                rot_vel_z: motion_data.rotVelZ as f64,
            }
        }
    }

    #[napi(object)]
    pub struct AnalogActionVector {
        pub x: f64,
        pub y: f64,
    }

    #[napi]
    pub fn init() {
        let client = crate::client::get_client().unwrap();
        client.input().init(false);
    }

    #[napi]
    pub fn get_controllers() -> Vec<Controller> {
        let client = crate::client::get_client().unwrap();
        client
            .input()
            .get_connected_controllers()
            .into_iter()
            .filter(|identity| identity != &0)
            .map(|identity| Controller {
                handle: BigInt::from(identity),
            })
            .collect()
    }

    #[napi]
    pub fn get_action_set(action_set_name: String) -> BigInt {
        let client = crate::client::get_client().unwrap();
        BigInt::from(client.input().get_action_set_handle(&action_set_name))
    }

    #[napi]
    pub fn get_digital_action(action_name: String) -> BigInt {
        let client = crate::client::get_client().unwrap();
        BigInt::from(client.input().get_digital_action_handle(&action_name))
    }

    #[napi]
    pub fn get_analog_action(action_name: String) -> BigInt {
        let client = crate::client::get_client().unwrap();
        BigInt::from(client.input().get_analog_action_handle(&action_name))
    }

    #[napi]
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum KeyboardInputKind {
        /// The text input that will closes on Enter keypress
        SingleLine,
        /// Text input that will closes on user's demand.
        MultiLine,
        /// Text input that will make typing email address easier.
        Email,
        /// Text input that will make typing numeric input easier.
        Numeric,
    }

    impl Into<FloatingGamepadTextInputMode> for KeyboardInputKind {
        fn into(self) -> FloatingGamepadTextInputMode {
            match self {
                KeyboardInputKind::SingleLine => FloatingGamepadTextInputMode::SingleLine,
                KeyboardInputKind::MultiLine => FloatingGamepadTextInputMode::MultipleLines,
                KeyboardInputKind::Email => FloatingGamepadTextInputMode::Email,
                KeyboardInputKind::Numeric => FloatingGamepadTextInputMode::Numeric,
            }
        }
    }


    /// Opens a floating keyboard over the game content and sends OS keyboard keys directly to the game.
    /// The text field position is specified in pixels relative the origin of the game window and is used to position the floating keyboard in a way that doesn't cover the text field.
    #[napi]
    pub fn trigger_on_screen_keyboard(keyboard_input_kind: KeyboardInputKind, x_pos_of_text_input: i32, y_pos_of_text_input: i32, width_of_text_input: i32, height_of_text_input: i32, dismissed_callback: Option<Function<'static>>) -> bool {
        let client = crate::client::get_client().unwrap();
        let out: Box<dyn FnMut() + Send + 'static> = if let Some(cb) = dismissed_callback {
            let threadsafe = cb.build_threadsafe_function().build_callback(|_| Ok(())).unwrap();
            Box::new(move || {
                threadsafe.call((), napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking);
            })
        } else {
            Box::new(|| {})
        };
        client.utils().show_floating_gamepad_text_input(keyboard_input_kind.into(), x_pos_of_text_input, y_pos_of_text_input, width_of_text_input, height_of_text_input, out)
    }

    #[napi]
    pub fn shutdown() {
        let client = crate::client::get_client().unwrap();
        client.input().shutdown()
    }

    #[napi]
    pub fn set_input_action_manifest_file_path(path: String) -> napi::Result<bool> {
        let path = CString::new(path).map_err(|a| napi::Error::from_reason(a.to_string()))?;
        unsafe {
            let x = steamworks::sys::SteamAPI_SteamInput_v006();
            return Ok(
                steamworks::sys::SteamAPI_ISteamInput_SetInputActionManifestFilePath(
                    x,
                    path.as_ptr(),
                ),
            );
        }
    }
}
