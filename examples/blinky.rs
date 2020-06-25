#![no_std]
#![no_main]

use libdaisy_rust::*;
use stm32h7xx_hal::stm32::TIM2;
use stm32h7xx_hal::timer::Timer;

#[rtic::app(device = stm32h7xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        timer2: Timer<TIM2>,
        seed_led: gpio::SeedLed,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        let system = system::System::init(ctx.core, ctx.device);

        init::LateResources {
            timer2: system.timer2,
            seed_led: system.gpio.led,
         }
    }

    #[task( binds = TIM2, priority = 1, resources = [timer2, seed_led] )]
    fn main(ctx: main::Context) {
        static mut LED_IS_ON: bool = false;
        ctx.resources.timer2.clear_irq();
        if *LED_IS_ON {
            ctx.resources.seed_led.set_high().unwrap();
        } else {
            ctx.resources.seed_led.set_low().unwrap();
        }
        *LED_IS_ON = !(*LED_IS_ON);
    }
};
