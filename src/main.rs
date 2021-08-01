#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER;

#[entry]
fn main() -> ! {
   loop {}
}
