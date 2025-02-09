use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub const CONST_INT: i32 = 42;
pub const CONST_ARRAY: [f32; 3] = [1.5, 3.0, 6.0];
