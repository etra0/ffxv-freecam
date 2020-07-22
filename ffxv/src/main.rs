use common::common::{Camera, Injection};
use memory_rs::process::process_wrapper::Process;
use std::f32;
use std::io::Error;
use std::thread;
use std::time::{Duration, Instant};
use winapi::shared::windef::POINT;
use winapi::um::winuser;
use winapi::um::winuser::{GetAsyncKeyState, GetCursorPos, SetCursorPos};
use winapi::um::xinput::*;

const INITIAL_POS: i32 = 500;

extern "C" {
    static get_camera_data: u8;
    static get_camera_data_end: u8;
}


pub fn main() -> Result<(), Error> {
    let mut mouse_pos: POINT = POINT::default();

    // latest mouse positions
    let mut latest_x = 0;
    let mut latest_y = 0;

    println!();

    println!("Waiting for the game to start");
    let process = loop {
        match Process::new("ffxv_u.exe") {
            Ok(p) => break p,
            Err(_) => (),
        }

        thread::sleep(Duration::from_secs(5));
    };
    println!("Game hooked");

    let entry_point: usize = 0x10FCAD7;
    let p_shellcode = unsafe {
        process.inject_shellcode(
            entry_point,
            7,
            &get_camera_data as *const u8,
            &get_camera_data_end as *const u8,
        )
    };

    let mut cam = Camera::new(&process, p_shellcode);

    let mut active = false;
    let mut capture_mouse = false;

    let mut restart_mouse = false;

    // nop other camera writers
    cam.injections.push(Injection {
        entry_point: 0x10FCADE,
        f_orig: vec![0xFF, 0x90, 0x50, 0x12, 0x00, 0x00],
        f_rep: vec![0x90; 6]
    });

    cam.injections.push(Injection {
        entry_point: 0x10FCC3F,
        f_orig: vec![0x0f, 0x29, 0x87, 0x20, 0x38, 0x00, 0x00],
        f_rep: vec![0x90; 7]
    });

    cam.injections.push(Injection {
        entry_point: 0x10FCC4D,
        f_orig: vec![0x41, 0x0f, 0x29, 0x07],
        f_rep: vec![0x90; 4]
    });

    let mut controller_state = XINPUT_STATE::default();

    loop {
        if capture_mouse & restart_mouse {
            unsafe { SetCursorPos(INITIAL_POS, INITIAL_POS) };
            restart_mouse = !restart_mouse;
            latest_x = INITIAL_POS;
            latest_y = INITIAL_POS;
            continue;
        }

        unsafe {
            XInputGetState(0, &mut controller_state);
        }


        let start = Instant::now();

        // poll rate
        thread::sleep(Duration::from_millis(10));
        unsafe { GetCursorPos(&mut mouse_pos) };
        let duration = start.elapsed().as_millis() as f32;

        // old way of doing it 
        // let speed_x = ((mouse_pos.x - latest_x) as f32) / duration/1.;
        // let speed_y = ((mouse_pos.y - latest_y) as f32) / duration/1.;

        if active  {
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

            cam.update_position(rx, -ry);
            cam.update_values(ly, -lx, 0., 0, 0, 0);
        }

        latest_x = mouse_pos.x;
        latest_y = mouse_pos.y;

        // debug purposes
        let focus = process.read_value::<[f32; 3]>(p_shellcode + 0x200, true);
        let position = process.read_value::<[f32; 3]>(p_shellcode + 0x200, true);

        println!("{:?}\t{:?}", focus, position);

        // to scroll infinitely
        restart_mouse = !restart_mouse;
        unsafe {
            if (controller_state.Gamepad.wButtons & 0x280) == 0x280
            {
                active = !active;
                capture_mouse = active;

                let c_status = if active { "Deattached" } else { "Attached" };
                println!("status of camera: {}", c_status);
                if active {
                    cam.deattach();
                } else {
                    cam.attach();
                }
                thread::sleep(Duration::from_millis(500));
            }

            if active & (GetAsyncKeyState(winuser::VK_DELETE) as u32 & 0x8000 != 0) {
                capture_mouse = !capture_mouse;
                let c_status = if !capture_mouse {
                    "Deattached"
                } else {
                    "Attached"
                };
                println!("status of mouse: {}", c_status);
                thread::sleep(Duration::from_millis(500));
            }
        }
    }
}
