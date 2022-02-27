
pub trait CountDown {
    /// The unit of time used by this timer
    type Time;

    /// Starts a new count down
    fn start<T>(&mut self, count: T)
    where
        T: Into<Self::Time>;
        //nb: non-blocking library
    fn wait(&mut self) -> Result<(), ()>;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
