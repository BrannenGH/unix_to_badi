extern crate time;

mod unix_to_badi;
mod badi_to_unix;

pub struct UnixTimesOfDay{
    sunrise: time::Timespec,
    noon: time::Timespec,
    sunset: time::Timespec,
}

pub struct BadiTime {
    before_gregorian_switch: bool,
    year: u32,
    month: u8,
    day: u8,
    day_of_week: u8,
}

pub mod badi_support{

}
