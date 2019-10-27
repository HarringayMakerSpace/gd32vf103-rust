#![no_std]
#![no_main]

extern crate panic_halt;

use gd32vf103_hal as hal;
use hal::pac;
use hal::prelude::*;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    // let mut gpioc = dp.GPIOC.split(&mut rcu.apb2);
    // let mut pc13 = gpioc.pc13.into_push_pull_output(&mut gpioc.ctl1);
    // pc13.set_low().unwrap();
    // let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);
    // pa1.set_low().unwrap();
    let mut pa2 = gpioa.pa2.into_push_pull_output(&mut gpioa.ctl0);
    pa2.set_low().unwrap();
    loop {}
}
