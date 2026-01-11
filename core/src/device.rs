use windows::Win32::UI::Input::*;
use windows::Win32::Foundation::*;
use windows::core::*;
use windows::Win32::UI::Input::{RID_DEVICE_INFO_TYPE, RIM_TYPEMOUSE};

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub handle: HANDLE,
    pub name: String,
    pub vid: u16,
    pub pid: u16,
}

pub fn enumerate_mice() -> Vec<DeviceInfo> {
    let mut devices = Vec::new();

    unsafe {
        // Anzahl der RawInput-Geräte ermitteln
        let mut num_devices = 0u32;
        if GetRawInputDeviceList(None, &mut num_devices, std::mem::size_of::<RAWINPUTDEVICELIST>() as u32) != 0 {
            return devices; // Fehler, leer zurück
        }

        if num_devices == 0 {
            return devices;
        }

        // Liste der Geräte abrufen
        let mut device_list: Vec<RAWINPUTDEVICELIST> = vec![std::mem::zeroed(); num_devices as usize];
        GetRawInputDeviceList(Some(device_list.as_mut_ptr()), &mut num_devices, std::mem::size_of::<RAWINPUTDEVICELIST>() as u32);

        for dev in &device_list {
            // Nur Mäuse
            if dev.dwType != RIM_TYPEMOUSE {
                continue;
            }

            // Gerätename abfragen
            let mut name_len = 0u32;
            GetRawInputDeviceInfoA(dev.hDevice, RIDI_DEVICENAME, None, &mut name_len);

            if name_len == 0 {
                continue;
            }

            let mut name_buf = vec![0u8; name_len as usize];
            GetRawInputDeviceInfoA(dev.hDevice, RIDI_DEVICENAME, Some(name_buf.as_mut_ptr() as *mut _), &mut name_len);

            let name = String::from_utf8_lossy(&name_buf).to_string();

            // Optional: VID/PID aus Name extrahieren
            let mut vid = 0;
            let mut pid = 0;
            if let Some(vid_str) = name.split("&").find(|s| s.starts_with("VID_")) {
                vid = u16::from_str_radix(&vid_str[4..], 16).unwrap_or(0);
            }
            if let Some(pid_str) = name.split("&").find(|s| s.starts_with("PID_")) {
                pid = u16::from_str_radix(&pid_str[4..], 16).unwrap_or(0);
            }

            devices.push(DeviceInfo {
                handle: dev.hDevice,
                name,
                vid,
                pid,
            });
        }
    }

    devices
}
