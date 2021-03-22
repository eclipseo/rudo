use libc::{isatty, ttyname};
use std::env;
use std::error::Error;
use std::ffi::CStr;

/// Safe wrapper to get the name of the current tty
/// and return it as a Rust string
pub fn get_tty_name() -> Result<String, Box<dyn Error>> {
    unsafe {
        if isatty(0) == 0 {
            return Err(From::from("Rudo must currently be called from a TTY!"));
        }
        let ttyname_c = ttyname(0);
        // Verify that call didn't fail
        if ttyname_c.is_null() {
            return Err(From::from("ttyname() call failed!"));
        }
        let ttyname_rust = CStr::from_ptr(ttyname_c).to_string_lossy().into_owned();
        Ok(ttyname_rust)
    }
}

pub fn tty_uuid() -> Result<String, Box<dyn Error>> {
    if env::var("GNOME_TERMINAL_SCREEN").is_ok() {
        let uuid = env::var("GNOME_TERMINAL_SCREEN")?;
        return Ok(uuid);
    }
    error!("Couldn't determine the terminal uuid");
    Err(From::from("Couldn't determine the terminal uuid"))
}
