use std::ffi::c_void;
use libloading::{Library, Symbol};

#[repr(C)]
pub struct VigemClient(c_void);

#[repr(C)]
pub struct Xbox360Report {
    pub wButtons: u16,
    pub bLeftTrigger: u8,
    pub bRightTrigger: u8,
    pub sThumbLX: i16,
    pub sThumbLY: i16,
    pub sThumbRX: i16,
    pub sThumbRY: i16,
}

pub struct VirtualController {
    client: *mut VigemClient,
    target: *mut c_void,
    // Wir halten die Library, damit sie nicht dropped wird
    _lib: Library,
}

impl VirtualController {
    /// 🎮 Erzeugt einen virtuellen Controller, dynamisch aus der DLL
    pub unsafe fn new() -> anyhow::Result<Self> {
        // DLL laden
        let lib = Library::new("E:/VSProjekte/motion-hid-bridge/libs/ViGEmClient.dll")?;

        // Funktionen aus der DLL holen
        let vigem_alloc: Symbol<unsafe extern "C" fn() -> *mut VigemClient> =
            lib.get(b"vigem_alloc")?;
        let vigem_connect: Symbol<unsafe extern "C" fn(*mut VigemClient) -> i32> =
            lib.get(b"vigem_connect")?;
        let vigem_target_x360_alloc: Symbol<unsafe extern "C" fn() -> *mut c_void> =
            lib.get(b"vigem_target_x360_alloc")?;
        let vigem_target_add: Symbol<unsafe extern "C" fn(*mut VigemClient, *mut c_void) -> i32> =
            lib.get(b"vigem_target_add")?;
        let vigem_target_x360_update: Symbol<unsafe extern "C" fn(*mut VigemClient, *mut c_void, Xbox360Report) -> i32> =
            lib.get(b"vigem_target_x360_update")?;

        // Client erzeugen und verbinden
        let client = vigem_alloc();
        vigem_connect(client);

        // Target erzeugen und hinzufügen
        let target = vigem_target_x360_alloc();
        vigem_target_add(client, target);

        Ok(Self { client, target, _lib: lib })
    }

    /// Vorwärts-/Rückwärtsbewegung setzen
    pub unsafe fn set_speed(&self, speed: f32) -> anyhow::Result<()> {
        let ly = (speed.clamp(-1.0, 1.0) * 32767.0) as i16;

        let report = Xbox360Report {
            wButtons: 0,
            bLeftTrigger: 0,
            bRightTrigger: 0,
            sThumbLX: 0,
            sThumbLY: ly,
            sThumbRX: 0,
            sThumbRY: 0,
        };

        // DLL-Funktion dynamisch aufrufen
        let vigem_target_x360_update: Symbol<unsafe extern "C" fn(*mut VigemClient, *mut c_void, Xbox360Report) -> i32> =
            self._lib.get(b"vigem_target_x360_update")?;
        vigem_target_x360_update(self.client, self.target, report);

        Ok(())
    }
}
