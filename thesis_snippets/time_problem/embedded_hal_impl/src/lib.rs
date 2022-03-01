use embedded_hal_declaration::CountDown;
use time_lib::Time_Lib_1;
pub struct EMBEDDED_TIMER;
impl CountDown for EMBEDDED_TIMER{
    type Time= Time_Lib_1::nanoseconds;
    fn start<T>(&mut self, count: T) where T: Into<Self::Time>{

        //sets the timer
    }
    fn wait(&mut self)->Result<(),()>{
        // not implemented for brevity
        return Ok(());
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
