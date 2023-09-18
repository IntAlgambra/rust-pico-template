#![no_std]
#![no_main]

// macros for creating entry point for function
use rp_pico::entry;


// we need panic handler (from 3rd party)
use panic_halt as _;


#[entry]
fn main() -> ! {
    loop {}
}