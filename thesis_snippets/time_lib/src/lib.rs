pub mod Time_Lib_1{
    pub enum Times{
        Milliseconds(u32),
        Microseconds(u32),
        Nanoseconds(u32),
    }

    impl From<Times> for nanoseconds{
        fn from(t: Times) -> Self {
            nanoseconds(match t{
                Times::Milliseconds(x)=>x*1000000,
                Times::Microseconds(x)=>x*1000,
                Times::Nanoseconds(x)=>x
            })
        }
    }
    pub struct nanoseconds(u32);


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
