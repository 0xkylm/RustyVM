// vm.rs
enum Bytc {
    MOV, ADD, SUB, INC, DEC
}
pub struct VM {
    pub eip: i32,
    pub current_bc: i32,
    pub arg1: i32,
    pub arg2: i32,
    pub binary: i32,
    pub full_byc: Vec<i32>,

    reg: REG,


}

struct REG{
    reg: [i32;10],
    reg_str: Vec<String>,
}




pub fn create_vm(buf: Vec<i32> ) -> Box<VM> {
    let vm = VM {
        eip: 0,
        current_bc: 0,
        arg1: 0,
        arg2: 0,
        binary: 0,
        full_byc: buf,
        reg: REG {
            reg: [0; 10], 
            reg_str: vec![], 
        },
    };
    Box::new(vm)
}

pub fn vm_loop(mut vm: Box<VM>) {
    while vm.eip < vm.full_byc.len() as i32 {
            vm.current_bc = vm.full_byc[vm.eip as usize];
        
        vm = giga_switch(vm);
        

    }

    print!("fini");
}

fn giga_switch(mut vm: Box<VM>) -> Box<VM> {
    //Mov,ADD,SUB,INC,DEC

    match vm.current_bc {

        1 =>{
            vm.arg1 = vm.full_byc[vm.eip as usize];
            
            vm.arg2 = vm.full_byc[vm.eip as usize];


            
        }
        2 =>{
            vm.eip +=1;
            vm.arg1 = vm.full_byc[vm.eip as usize];
            vm.eip +=1;
            vm.arg2 = vm.full_byc[vm.eip as usize];
            

            print!("I'll add {} with {} it's equal = {}\n",vm.reg.reg[vm.arg1 as usize],vm.reg.reg[vm.arg2 as usize], vm.reg.reg[vm.arg1 as usize] + vm.reg.reg[vm.arg2 as usize]);
            
            vm.reg.reg[vm.arg1 as usize] = vm.reg.reg[vm.arg1 as usize] + vm.reg.reg[vm.arg2 as usize];

            vm.eip += 1;
            
        }
        3 =>{
            vm.eip +=1;
            vm.arg1 = vm.full_byc[vm.eip as usize];
            vm.eip +=1;
            vm.arg2 = vm.full_byc[vm.eip as usize];



            vm.eip += 1;
        }


        4 =>{
        vm.eip +=1;
        vm.arg1 = vm.full_byc[vm.eip as usize];
        vm.eip +=1;
        vm.arg2 = vm.full_byc[vm.eip as usize];

        vm.reg.reg[ vm.arg1 as usize] += vm.arg2;

        vm.eip += 1;
       } 
       5 =>{
        vm.eip +=1;
        vm.arg1 = vm.full_byc[vm.eip as usize];
        vm.eip +=1;
        vm.arg2 = vm.full_byc[vm.eip as usize];

        vm.reg.reg[ vm.arg1 as usize] -= vm.arg2;

        vm.eip += 1;
       } 

        _ => {
            println!("Error reading bc = {}", vm.current_bc);
           // vm.eip += 1;
        }
    }
    vm
}
