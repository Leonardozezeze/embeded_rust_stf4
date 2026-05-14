#![no_main]
#![no_std]

use embeded_rust_stf4 as _; // global logger + panic handler + memory layout

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use defmt::println;
use stm32f4xx_hal::{self as hal, rcc::Config};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // 配置时钟：外部 HSE 8MHz，PLL 到 168MHz
    let mut rcc = dp.RCC.freeze(Config::hse(8.MHz()).sysclk(168.MHz()));

    // LED: PD12（STM32F4-Discovery 板载 LED）
    let gpiod = dp.GPIOD.split(&mut rcc);
    let mut led = gpiod.pd12.into_push_pull_output();

    // SysTick 延时
    let mut delay = cp.SYST.delay(&rcc.clocks);

    println!("STM32F407 booted!");

    loop {
        led.toggle();
        delay.delay_ms(500u32);
    }
}
