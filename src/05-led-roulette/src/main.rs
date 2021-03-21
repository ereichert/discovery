#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};
use volatile::Volatile;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    let mut overlap = 50_u16;
    let v_overlap = Volatile::new(&mut overlap);

    loop {
        leds[LedOrientation::N].on().ok();
        leds[LedOrientation::NE].on().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::N].off().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::E].on().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::NE].off().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::SE].on().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::E].off().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::S].on().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::SE].off().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::SW].on().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::S].off().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::W].on().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::SW].off().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::NW].on().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::W].off().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::N].on().ok();
        delay.delay_ms(v_overlap.read());

        leds[LedOrientation::NW].off().ok();
        delay.delay_ms(v_overlap.read());
    }
}

struct LedOrientation;
impl LedOrientation {
    pub const N: usize = 0;
    pub const NE: usize = 1;
    pub const E: usize = 2;
    pub const SE: usize = 3;
    pub const S: usize = 4;
    pub const SW: usize = 5;
    pub const W: usize = 6;
    pub const NW: usize = 7;
}
