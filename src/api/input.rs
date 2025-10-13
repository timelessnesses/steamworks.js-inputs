use napi_derive::napi;

#[napi]
pub mod input {
    use napi::bindgen_prelude::BigInt;

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
    pub struct Controller {
        pub(crate) handle: BigInt,
    }

    #[napi]
    impl Controller {
        #[napi]
        pub fn activate_action_set(&self, action_set_handle: BigInt) {
            let client = crate::client::get_client();
            client
                .input()
                .activate_action_set_handle(self.handle.get_u64().1, action_set_handle.get_u64().1)
        }

        #[napi]
        pub fn is_digital_action_pressed(&self, action_handle: BigInt) -> bool {
            let client = crate::client::get_client();
            client
                .input()
                .get_digital_action_data(self.handle.get_u64().1, action_handle.get_u64().1)
                .bState
        }

        #[napi]
        pub fn get_analog_action_vector(&self, action_handle: BigInt) -> AnalogActionVector {
            let client = crate::client::get_client();
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
            let client = crate::client::get_client();
            client
                .input()
                .get_input_type_for_handle(self.handle.get_u64().1)
                .into()
        }

        #[napi]
        pub fn get_handle(&self) -> BigInt {
            self.handle.clone()
        }

        /// Gets controller latest data, best use for low latency if you call this all the time
        #[napi]
        pub fn run_frame(&self) {
            let client = crate::client::get_client();
            client.input().run_frame();
        }

        /// Gets controller's motion sensors
        #[napi]
        pub fn get_motion_data(&self) -> Option<MotionData> {
            let client = crate::client::get_client();
            let a =client
                .input()
                .get_motion_data(self.handle.get_u64().1);
            let quat_is_identity = a.rotQuatX == 0.0 && a.rotQuatY == 0.0 && a.rotQuatZ == 0.0 && a.rotQuatW == 1.0;
            let accel_and_rot_vel_are_zero = a.posAccelX == 0.0 && a.posAccelY == 0.0 && a.posAccelZ == 0.0 && a.rotVelX == 0.0 && a.rotVelY == 0.0 && a.rotVelZ == 0.0;
            if quat_is_identity && accel_and_rot_vel_are_zero {
                None
            } else {
                Some(a.into())
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
        let client = crate::client::get_client();
        client.input().init(false);
    }

    #[napi]
    pub fn get_controllers() -> Vec<Controller> {
        let client = crate::client::get_client();
        client
            .input()
            .get_connected_controllers()
            .into_iter()
            .map(|identity| Controller {
                handle: BigInt::from(identity),
            })
            .collect()
    }

    #[napi]
    pub fn get_action_set(action_set_name: String) -> BigInt {
        let client = crate::client::get_client();
        BigInt::from(client.input().get_action_set_handle(&action_set_name))
    }

    #[napi]
    pub fn get_digital_action(action_name: String) -> BigInt {
        let client = crate::client::get_client();
        BigInt::from(client.input().get_digital_action_handle(&action_name))
    }

    #[napi]
    pub fn get_analog_action(action_name: String) -> BigInt {
        let client = crate::client::get_client();
        BigInt::from(client.input().get_analog_action_handle(&action_name))
    }

    #[napi]
    pub fn shutdown() {
        let client = crate::client::get_client();
        client.input().shutdown()
    }
}
