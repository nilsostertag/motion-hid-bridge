mod device;
mod input;
mod motion;
mod emulator;

use crate::device::*;
use crate::input::*;
use crate::emulator::VirtualController;

use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::Input::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::s;

use std::io::{self, Write};

use anyhow::Result; // Für das Result in main

fn main() -> Result<()> {
    unsafe {
        // -------------------------------------------------
        // 1️⃣ Message-only Window für Raw Input
        // -------------------------------------------------
        let h_instance = GetModuleHandleA(None)?;

        let hwnd = CreateWindowExA(
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

        // -------------------------------------------------
        // 2️⃣ Mäuse auflisten & auswählen
        // -------------------------------------------------
        let mice = enumerate_mice();
        if mice.is_empty() {
            println!("Keine Mäuse gefunden.");
            return Ok(());
        }

        println!("Verfügbare Mäuse:");
        for (i, m) in mice.iter().enumerate() {
            println!(
                "{}: {} (VID:{:04X} PID:{:04X})",
                i + 1,
                m.name,
                m.vid,
                m.pid
            );
        }

        print!("Wähle Maus (Nummer): ");
        io::stdout().flush().unwrap();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let index: usize = input_line.trim().parse().unwrap_or(1);

        let selected_mouse = mice
            .get(index.saturating_sub(1))
            .cloned()
            .expect("Ungültige Auswahl");

        println!("Ausgewählte Maus: {}", selected_mouse.name);

        // -------------------------------------------------
        // 3️⃣ MouseInput initialisieren
        // -------------------------------------------------
        let mut mouse_input = MouseInput::new();
        mouse_input.select_device(selected_mouse);

        // -------------------------------------------------
        // 4️⃣ Raw Input registrieren
        // -------------------------------------------------
        let rid = RAWINPUTDEVICE {
            usUsagePage: 0x01, // Generic Desktop
            usUsage: 0x02,     // Mouse
            dwFlags: RIDEV_INPUTSINK,
            hwndTarget: hwnd,
        };

        RegisterRawInputDevices(
            &[rid],
            std::mem::size_of::<RAWINPUTDEVICE>() as u32,
        )?;

        // -------------------------------------------------
        // 5️⃣ Virtuellen Controller erzeugen
        // -------------------------------------------------
        let controller = unsafe { VirtualController::new()? };
        println!("Virtueller Xbox-Controller erzeugt.");

        println!("motion-hid-bridge aktiv.");

        // -------------------------------------------------
        // 6️⃣ Message Loop
        // -------------------------------------------------
        let mut msg = MSG::default();
        while GetMessageA(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);

            if msg.message == WM_INPUT {
                if let Some(speed_mps) = mouse_input.handle_raw_input(msg.lParam) {
                    // Beispiel: 2.5 m/s = voller Stick
                    let normalized = (speed_mps / 2.5).clamp(-1.0, 1.0);

                    unsafe {
                        controller.set_speed(normalized)?;
                    }

                    println!(
                        "Speed: {:>6.2} m/s | StickY: {:>5.2}",
                        speed_mps,
                        normalized
                    );
                }
            }
        }
    }

    Ok(())
}
