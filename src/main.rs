#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;
use stm32f4xx_hal as _; // 取消注释这行

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true)
    }
}
