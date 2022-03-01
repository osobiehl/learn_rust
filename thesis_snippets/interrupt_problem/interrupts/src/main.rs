#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use core::{cell::RefCell, mem::replace, borrow::Borrow, ops::Deref, };
use cortex_m_rt::{entry};
use cortex_m::interrupt::{ self as intrr, Mutex};
use stm32f1::stm32f103::interrupt;
//example peripheral
struct Peripheral(());
impl Peripheral{
    fn foo(&self)->usize{5}
    fn init()->Option<Peripheral>{
        return Some(Peripheral(()) )  ;
    }
}
static MY_PERIPHERAL: Peri =
    Mutex::new(RefCell::new(None));

struct Peripheral_Wrapper(Mutex<RefCell<Option<Peripheral>>> )
#[entry]
fn main()->!{
    let peripheral = Peripheral::init().unwrap();
    let ref_peripheral = &peripheral;
    intrr::free(|critical_section| MY_PERIPHERAL.borrow(critical_section).replace(Some(peripheral)));
    // to interact with the peripheral
    intrr::free(|critical_section| {
    let peripheral = MY_PERIPHERAL.borrow(critical_section).borrow_mut();
    let x =  peripheral.as_ref().unwrap().foo();
    

    });
    loop{}
}
#[interrupt]
fn EXTI1(){
    static moved_periph: RefCell<Option<Peripheral>> = RefCell::new(None);

    intrr::free(|critical_section| {
        let mut  peripheral_mutex = MY_PERIPHERAL.borrow(critical_section);
        let periph = peripheral_mutex.borrow_mut();
        if let Some(p) = periph.deref(){
            moved_periph= replace(&mut peripheral_mutex, RefCell::new(None))
        }
    });

}

