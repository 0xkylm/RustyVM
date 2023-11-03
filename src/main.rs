
mod vm; 
use vm::{VM, create_vm};

use crate::vm::vm_loop; 

fn main() {
    let buf: Vec<i32> = vec![4,2,2,4,1,4,2,2,2];
    let my_vm = create_vm(buf); 
    vm_loop(my_vm);
}