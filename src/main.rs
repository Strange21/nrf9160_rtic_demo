#![no_main]
#![no_std]

#[rtic::app(device = nrf9160_hal::pac, peripherals = true)]//dispatchers = [UARTE0_SPIM0_SPIS0_TWIM0_TWIS0]
mod app {

    use nrf9160_hal::{
        gpio::{Level, Output, Pin, PushPull},
        prelude::*,
        pac::{Interrupt, UARTE0_NS},
        pac::P0_NS,
        uarte,
        gpio, Timer, Uarte
    };
    use cortex_m::asm::delay;
    use core::fmt::Write;

    #[local]
    struct Local {
        led1: Pin<Output<PushPull>>,
        led2: Pin<Output<PushPull>>,
        // uarte: Uarte<UARTE0_NS>,
    }

    #[shared]
    struct Shared {

    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let peripherals = cx.device;

        // Initialize LED pin as output
        let p0: P0_NS = unsafe { core::mem::transmute(()) };
        let p0 = gpio::p0::Parts::new(p0);
        
        let mut led1 = p0.p0_03.into_push_pull_output(gpio::Level::Low).degrade();
        let mut led2 = p0.p0_04.into_push_pull_output(gpio::Level::Low).degrade();
        
        // initialize timer0 for generating interrupt for each 1 sec 
        let mut timer = Timer::new(peripherals.TIMER0_NS);
        timer.enable_interrupt();
        timer.start(10_000_000u32);
        rtic::pend(Interrupt::TIMER0);
        // toggle_led::spawn().unwrap();
        (
            Shared{},
            Local{
                led1,
                led2,   
            },
            init::Monotonics()
        )
    }

    #[idle(local = [led1])]
    fn idle(mut cx: idle::Context)-> ! {
        rtic::pend(Interrupt::TIMER0);
        // Toggle LED
        cx.local.led1.set_high().unwrap();
        loop{   }
    }

    #[task(binds = TIMER0, local = [led2])]
    fn toggle_led(mut cx: toggle_led::Context) {
        match cx.local.led2.is_set_high().unwrap() {
            true => {
                cx.local.led2.set_low().unwrap();
            }

            false => {
                cx.local.led2.set_high().unwrap();
            }
        }
        delay(5_000_000);    
    }
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
