consider a project with the following structure:
```shell
├── app
├── embedded_hal_declaration
├── embedded_hal_impl
├── time_lib
```
Each of these directories represents a separate Rust crate. For the sake of brevity, only the CountDown trait is considered and used to write impl blocks. The code is as follows
## embedded_hal_declaration
```rust
/// as defined in the embedded_hal crate before being removed for version 1.0.0
pub trait CountDown {
    /// The unit of time used by this timer
    type Time;

    /// Starts a new count down
    fn start<T>(&mut self, count: T)
    where
        T: Into<Self::Time>;
        //normally uses non-blocking library, removed for brevity
    fn wait(&mut self) -> Result<(), ()>;
}

```

## time_lib
The intent of this time library snippet is to show a plausible way in which a timer could be implemented in hardware. Although the base unit for the timer is a `u32` wrapper called nanoseconds, the module abstracts it behind an enum to improve programmer experience and reduce the likelihood of errors caused by misreading the documentation or incorrect assumptions. 
```rust
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
```

## embedded_hal_impl
Since this example is meant to be simple, it is easy to notice that the dependency on the time library could be removed by simply using the `u32` nanosecond variable it wraps. Fully operational time libraries would probably add an additional layer of complexity by considering frequencies and the minimum resolution of the clock.
```rust
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
```
## app
The developer is now forced to use the time types defined in `embedded_hal_impl`, in this case using the library used by the HAL implementation. While this may seem like a minor inconvenience, it is problematic because it
-  Removes abstraction by forcing the use of a specific library
-  Quickly becomes cumbersome to work with if peripherals use a different underlying `time` type in the embedded HAL declaration
```rust
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
```
## solutions
Due to Rust's *orphan rule*, it is not allowed for developers to write impl blocks if the trait and the types are defined in different crates. Some of the basic solutions are:
-  Create a wrapper type around the time type they want to use, then write the impl blocks for the HAL using the wrapper type. This bypasses the orphan rule because it defines the new types in the same crate where the impl blocks will be written
-  Expose the primitive type used for the timer. this is usually of type `u32` or `u64` depending on the architecture used. This may work for simple clocks, but ones with dynamic precision and other features will not work.
-  Homogenize time into a library. This is currently being implemented by projects such as [embedded-time](https://github.com/FluenTech/embedded-time) have tried to provide a common library for working with time using various platforms, but the project seems to have lost steam (no commits since  OCT 2021). Other projects, such as [drogue-embedded-timer](https://github.com/drogue-iot/drogue-embedded-timer) also offer to provide macros to convert HAL-specific timers into generic `embedded-time` timers. However, The project seems somewhat abandoned (no commits since OCT 2020);