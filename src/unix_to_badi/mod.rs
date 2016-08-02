use extern crate time;

fn unix_to_badi(unix_time: Timespec, times: UnixTimesOfDay) -> BadiTime{
    // run the calculations
    let mut bahai_year = bahai_year(unix_time.sec);


    return BadiTime {
        year: bahai_year
    }
}

fn bahai_year(unix_time:i32) -> u32{
    const BAHAI_UNIX_YEAR: u32 =  
    return unix_time
}
