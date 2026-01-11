use std::time::Instant;
use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::*;
use crate::device::DeviceInfo;

/// Struktur für die Maus-Eingabe
pub struct MouseInput {
    /// Ausgewählte Maus
    pub selected: Option<DeviceInfo>,
    last_time: Instant,
}

impl MouseInput {
    /// Neue Instanz
    pub fn new() -> Self {
        Self {
            selected: None,
            last_time: Instant::now(),
        }
    }

    /// Verarbeitet Raw Input Events und gibt die Geschwindigkeit zurück (nur für ausgewählte Maus)
    pub unsafe fn handle_raw_input(&mut self, lparam: LPARAM) -> Option<f32> {
        // Größe des RawInput-Puffers ermitteln
        let mut size = 0u32;
        GetRawInputData(
            HRAWINPUT(lparam.0 as isize),
            RID_INPUT,
            None,
            &mut size,
            std::mem::size_of::<RAWINPUTHEADER>() as u32,
        );

        if size == 0 {
            return None;
        }

        // RawInput in Buffer laden
        let mut buffer = vec![0u8; size as usize];
        GetRawInputData(
            HRAWINPUT(lparam.0 as isize),
            RID_INPUT,
            Some(buffer.as_mut_ptr() as *mut _),
            &mut size,
            std::mem::size_of::<RAWINPUTHEADER>() as u32,
        );

        let raw = &*(buffer.as_ptr() as *const RAWINPUT);

        // Prüfen ob Maus-Event
        if RID_DEVICE_INFO_TYPE(raw.header.dwType) == RIM_TYPEMOUSE {
            // Wenn eine Maus ausgewählt wurde, andere ignorieren
            if let Some(device) = &self.selected {
                if raw.header.hDevice != device.handle {
                    return None;
                }
            }

            // ΔY und Geschwindigkeit berechnen
            let dy = -raw.data.mouse.lLastY;
            let now = Instant::now();
            let dt = now.duration_since(self.last_time);
            self.last_time = now;

            // Geschwindigkeit in m/s (vorläufig: 1 ΔY = 1 mm)
            Some(dy as f32 / 1000.0 / dt.as_secs_f32())
        } else {
            None
        }
    }

    /// Maus auswählen
    pub fn select_device(&mut self, device: DeviceInfo) {
        self.selected = Some(device);
    }
}
