
mod vm; 
use vm::create_vm;

use crate::vm::vm_loop; 

fn main() {
    let buf: Vec<i32> = vec![6,0x6C,0,6,0x73,1,7,6,0x6C,0,6,0x73,1,7];
    let  my_vm = create_vm(buf); 
    vm_loop(my_vm);
   
    
}