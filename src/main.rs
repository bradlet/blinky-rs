//! main.rs

#![no_std]
#![no_main]

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
	
	let mut is_on = false;

	rprintln!("Starting...");
    _board.display_pins.col3.set_low().unwrap();

    loop {
		timer.delay_ms(500u16);
		is_on = if is_on {
			_board.display_pins.row3.set_low().unwrap();
			rprintln!("low");
			false
		} else {
			_board.display_pins.row3.set_high().unwrap();
			rprintln!("high");
			true
		}
    }
}
