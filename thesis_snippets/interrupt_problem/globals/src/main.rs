use std::cell::UnsafeCell;
struct Global(UnsafeCell<u32>);
impl Global{
    fn set(&self, val: u32){
        unsafe {*self.0.get() = val};
    }
    fn get(&self)->u32{
        unsafe{*self.0.get()}
    }
}
// Sync must be implemented
unsafe impl  Sync for Global{}
static global_var: Global = Global(UnsafeCell::new(0));

fn main() {
    global_var.set(1000);
    println!("Global: {}", global_var.get());
}
