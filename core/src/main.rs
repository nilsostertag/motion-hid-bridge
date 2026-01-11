mod device;
mod input;

use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::Input::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::device::*;
use crate::input::*;
use windows::core::s;
use std::io::{self, Write};


fn main() -> windows::core::Result<()> {
    unsafe {
        let h_instance = GetModuleHandleA(None)?;

        let hwnd = windows::Win32::UI::WindowsAndMessaging::CreateWindowExA(
            WINDOW_EX_STYLE(0),
            s!("STATIC"),
            s!("RawInputWindow"),
            WS_OVERLAPPEDWINDOW,
            0, 0, 0, 0,
            HWND_MESSAGE,
            None,
            h_instance,
            None,
        );

        let mouse_input = MouseInput::new();

        let mice = enumerate_mice();
        println!("Verfügbare Mäuse:");
        for (i, m) in mice.iter().enumerate() {
            println!("{}: {} (VID:{:04X} PID:{:04X})", i + 1, m.name, m.vid, m.pid);
        }
        print!("Wähle Maus (Nummer): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let index: usize = input.trim().parse().unwrap_or(1); // Standard 1
        let selected_mouse = mice.get(index - 1).cloned();

        let mut mouse_input = MouseInput::new();
        if let Some(device) = selected_mouse {
            println!("Ausgewählte Maus: {}", device.name);
            mouse_input.select_device(device);
}

        // Raw Input registrieren
        let rid = RAWINPUTDEVICE {
            usUsagePage: 0x01,
            usUsage: 0x02,
            dwFlags: RIDEV_INPUTSINK,
            hwndTarget: hwnd,
        };
        RegisterRawInputDevices(&[rid], std::mem::size_of::<RAWINPUTDEVICE>() as u32)?;

        println!("motion-hid-bridge actice.");

        let mut msg = MSG::default();
        while GetMessageA(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);

            if msg.message == WM_INPUT {
                if let Some(speed) = mouse_input.handle_raw_input(msg.lParam) {
                    println!("Geschwindigkeit: {:.2}", speed);
                }
            }
        }
    }
    Ok(())
}
