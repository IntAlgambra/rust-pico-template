#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this cr ate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

// Some traits we need
use core::fmt::Write;
use hal::fugit::RateExtU32;
use rp2040_hal::clocks::Clock;

// UART related types
use hal::uart::{DataBits, StopBits, UartConfig};
use rp_pico::entry;
use usbd_serial::embedded_io::Read;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // uart stuff
    let uart_pins = (
        // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
        pins.gpio0.into_function(),
        // UART RX (characters received by RP2040) on pin 2 (GPIO1)
        pins.gpio1.into_function(),
    );
    let mut uart = hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(115200.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();

    loop {
        write!(uart, "connected\n\r").unwrap();
        // ждем сообщения по юарт что можно отправлять данные
        let mut buf: [u8; 1] = [0; 1];
        let mut got_command = false;
        match uart.read(&mut buf) {
            Ok(_) => {
                // d
                if buf[0] == 100 {
                    got_command = true;
                }
            }
            Err(_) => {}
        };
        if got_command {
            write!(uart, "pong\n\r").unwrap();
        }
    }
}
