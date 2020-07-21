#[cfg(target_os = "linux")]
static DRIVERFILELOCATION: &str = "/dev/input/by-path/platform-i8042-serio-0-event-kbd";

use std::fs::File;
use std::io::{Read};

use libc::time_t;
use libc::suseconds_t;

// Representation of the kbd event struct in C on linux
#[derive(Debug)]
#[repr(C)]
struct InputEvent {
    tv_sec: time_t,      
    tv_usec: suseconds_t,     
    pub type_: u16,
    pub code: u16,
    pub value: i32
}

// Contains a callback set 
// by the user and the character code the input system will trigger on
struct Listener<'a>{
    cb_function: &'a dyn Fn(),
    seeked_char: u8
}

// This is 
pub struct InputSystem<'a>
{
    driver_file: std::fs::File,
    listeners: Vec<Listener<'a>>,
}

impl InputSystem<'_>
{

    pub fn new() -> Result<Self, &'static str>
    {
         root_check();
        let ret = File::open(DRIVERFILELOCATION);
        let file = match ret {
            Ok(opened_file) => opened_file,
            Err(_err_code) => return Err("could not open specified driver file"),
        };
        let default: InputSystem = InputSystem{
            driver_file: file,
            listeners: Vec::new()
        };
        Ok(default)
    }

    fn read_event(&mut self) -> Result<InputEvent, ()>
    {
        let mut buf = [0u8; std::mem::size_of::<InputEvent>()];
        let ret: usize = self.driver_file.read(&mut buf).unwrap();
        if ret != std::mem::size_of::<InputEvent>() {
            panic!("Error while reading from device file");
        }
        let return_packet: InputEvent = unsafe {std::mem::transmute(buf)};
        Ok(return_packet)
    }

    pub fn read_key(&mut self) -> Option<u8>
    {
        let ev = self.read_event().unwrap();
        if ev.value == 1 || ev.value == 2
        {
            for i in &self.listeners
            {
                if i.seeked_char == ev.code as u8 {
                    (i.cb_function)();
                }
            }
            return Some(ev.code as u8);
        }
        else
        {
            return None;
        }
    }

    pub fn attach_listener(&mut self, key_code: u8, callback: &'static dyn Fn())
    {
        let ret = Listener{cb_function: callback, seeked_char: key_code};
        self.listeners.push(ret);
    }
}

fn root_check() {
    unsafe
    {
        if libc::geteuid() != 0 {
            panic!("Must run as root user");
        }
    }
}
