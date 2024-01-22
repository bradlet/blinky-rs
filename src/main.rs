#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{
	board::Board,
	hal::{
		prelude::*,
		Timer,
	},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut _board = Board::take().unwrap();
	let mut timer = Timer::new(_board.TIMER0);
	
	let mut isOff = false;

    _board.display_pins.col1.set_high().unwrap();

    loop {
		timer.delay_ms(300u16);
		isOff = if isOff {
			_board.display_pins.col1.set_high().unwrap();
			true
		} else {
			_board.display_pins.col1.set_low().unwrap();
			true
		}
    }
}
