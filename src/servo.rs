#![allow(dead_code)]  //removes some warnings for the user
//use std::Duration;
use rppal::pwm::{Channel,Polarity,Pwm};
use std::time::Duration;


struct Servo {
        pin: Pwm,
        pub min_us: u16,
        pub max_us: u16 ,
        pub neutral_us: u16,
        pub enable:bool ,
        pub duty_cycle: u16,

        channel: u8,
        polarity: bool ,
}


impl Servo {
    pub fn new(n_channel: u8) -> Self{
        let mut n2_ch = Channel::Pwm0;
        if n_channel == 0 {}
        if n_channel == 1 {n2_ch = Channel::Pwm1}
    let _pwm = Pwm::with_period(
            n2_ch,
            Duration::from_millis(20),
            Duration::from_micros(2450),
            Polarity::Normal,
            true,
        ).unwrap();
        Self{
            pin: _pwm,
            min_us: 500 ,
            max_us: 2450 ,
            neutral_us: 1500,
            enable: false ,
            duty_cycle: 20,

            channel: n_channel,
            polarity: true,
        }

    }


    pub fn write(&mut self, value:u8) -> Result<(),rppal::pwm::Error>{
        self.pin.set_pulse_width(Duration::from_micros(((500.0 + (value as f64)*11.11).floor()) as u64))

    }

}
