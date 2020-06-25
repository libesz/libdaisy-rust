// use embedded_hal;
use embedded_hal::digital::v2::InputPin;
use stm32h7xx_hal::gpio;
pub use stm32h7xx_hal::gpio::{Analog, Output, Input, PushPull, PullUp, PullDown};

pub enum Never {}

pub trait Switch<Mode: InputPin> {
    fn test(&self);
}

// impl <Mode> for dyn InputPin<Error = Never> {
//     fn test(&self) {

//     }
// }