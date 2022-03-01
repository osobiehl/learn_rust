## interrupts
in a general sense, interrupts are requests made to processors to interrupt the processor's control flow. Interrupts are divided into Hardware and Software interrupts. Hardware interrupts are associated to a specific device or function of a CPU, such as an internal timer for time interrupts, or interrupts received from peripheral devices. Software interrupts are usually associated to specific processor instructions (system calls) that relinquish control to the operating system to accomplish a task. In an embedded environment, this could happen when interacting with a device driver.
currently executing code (the control flow) (Silberschatz, Galvin, Gagne) (Operating System Concepts 8th edition).

## Concurrency problems in embedded systems
Most embedded systems use single-core processors. However, the existence of interrupts (and task scheduling in certain embedded Operating Systems) means that race conditions can occur when tasks (and interrupts) share memory. 
### Current Tools
-  Modern embedded-cpus provide tools to work with concurrency issues. For example, the ARM cortex-M processors contain the notion of critical sections, where it is guaranteed that interrupts will not preempt code execution inside the critical section. 
-  Certain CPUs also provide atomic types, where updating a value is guaranteed to occur in a single logical cycle, thus preventing race conditions from occurring if a process loads a value into registers, the value is then changed (in memory) by an interrupt, and then the process acts upon the value in the register and stores it again in its initial place, essentially overwriting the value stored by the interrupt.
### Rust's approach to dealing with Concurrency.
One of Rust's main appeals is its ability to prevent data races in safe rust. The rust language specification (Rustonomicon) defines data races as:
-  two or more threads concurrently accessing a location of memory
-  one or more of them is a write
-  one or more of them is unsynchronized
Rust's concurrency model allows for data to be shared if it implements the `Sync` trait, and allows data to be sent (moved) between threads if it implements the `Send` trait. Almost all primitive types in Rust implement `Send` and `Sync`, the traits are also automatically derived for user-defined structs as long as it is composed entirely of `Send` and `Sync` implementing types. (Rustonomicon)[https://doc.rust-lang.org/nomicon/send-and-sync.html]
Although data races are prevented by Rust, general race conditions are part of safe Rust. This means taht critical sections and mutexes should still be used when interacting with software interrupts, this is especially true due to the main issue with Rust's current 

## The interrupt problem
Taking the ARM cortex-M processors as the standard, interrupt handlers are functions that take no arguments. This is an issue because it means that all data interrupts must be module scoped Atomic variables, local static variables, or global variables. The open issue is looking to support the following use cases:
1.  Sharing a variable between the main thread and only one interrupt handler
2.  Sharing a variable between the main thread and one or more interrupt handlers
3.  Moving a variable from the main thread to one interrupt handler

Modifying global variables in Rust is *unsafe*. A basic way to interact with a global variable would be as such:

```rust
static mut COUNTER: u32 = 0;
#[entry]
fn main() -> ! {
    set_timer_1hz();
    let mut last_state = false;
    loop {
        let state = read_signal_level();
        if state && !last_state {
            // New critical section ensures synchronised access to COUNTER
            cortex_m::interrupt::free(|_| {
                unsafe { COUNTER += 1 };
            });
        }
        last_state = state;
    }
}
#[interrupt]
fn timer() {
    unsafe { COUNTER = 0; }
}
``` 
[source](https://docs.rust-embedded.org/book/concurrency/index.html)
This is clearly not an idiomatic way to write Rust code since it requires using the `unsafe` keyword whenever *counter* needs to be modified. This code works to fulfill condition **1**, since the main thread cannot preempt the interrupt handler. A more idiomatic approach would be to wrap the unsafe accesses to the global variable around a safe api e.g.: 
```rust

use core::cell::UnsafeCell;
use cortex_m::interrupt;

struct Global(UnsafeCell<u32>);
impl Global{
    //pass a critical section to ensure no data races occur
    fn set(&self, val: u32, _cs: &interrupt::CriticalSection){
        unsafe {*self.0.get() = val};
    }
    fn get(&self)->u32{
        unsafe{*self.0.get()}
    }
}
// Sync must be implemented for global variables
unsafe impl  Sync for Global{}
static global_var: Global = Global(UnsafeCell::new(0));
```
This generalization could be applied to generic types as long as they implement `Copy`. However, when working with types that are not `Copy`, such as peripherals, writing safe code can quickly become extremely unwieldy as borrow checks are now enforced at runtime instead of compile time. 
Consider the task of interacting with a peripheral in a software interrupt:
```rust
use core::cell::RefCell;
use cortex_m::interrupt::{self, Mutex};
//example peripheral
use Peripheral_Example::Peripheral;

static MY_PERIPHERAL: Mutex<RefCell<Option<Peripher_Example::Peripheral>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main()->!{
    let peripheral = Peripheral_Example::init().unwrap();
    let ref_peripheral = &peripheral;
    interrupt::free(|critical_section| MY_PERIPHERAL.borrow(cs).replace(Some(peripheral)));
    // to interact with the peripheral
    interrupt::free(|critical_section| {
    let peripheral = MY_PERIPHERAL.borrow(critical_section).borrow_mut();
    peripheral.as_ref().unwrap().do_something();

    });
        }
}
```
Note that the `Mutex` implementation for Cortex_M processors requires to be in a critical section to prevent deadlocks where the interrupt waits indefinitely for a resource taken by the main thread. While this approach is effective, it is un-idiomatic.
Applying condition **3** to this implementation is also doable by moving the data used for communication into a static variable on the interrupt function.
     


