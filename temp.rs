macro_rules! dead_zone {
    ($var:expr) => {{
        let t = f32::from($var);
        if t.abs() > 300_f32 {
            (t / 1e5)
        } else {
            0_f32
        }
    }}
}
let rx = dead_zone!(controller_state.Gamepad.sThumbRX);
let ry = dead_zone!(controller_state.Gamepad.sThumbRY);

let lx = dead_zone!(controller_state.Gamepad.sThumbLX)*100.;
let ly = dead_zone!(controller_state.Gamepad.sThumbLY)*100.;
