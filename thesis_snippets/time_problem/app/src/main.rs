
//using time lib 2 for time
use embedded_hal_impl::EMBEDDED_TIMER;
use embedded_hal_declaration::CountDown;
// now the user is forced to use whatever library was used to implement
// the HAL for the timer.
use time_lib::Time_Lib_1;
fn main() {
    let mut t = EMBEDDED_TIMER;
    t.start(Time_Lib_1::Times::Microseconds(100));
    //issues start arising when peripherals use other kinds of 
    //time types.
    println!("Hello, world!");
    
}
