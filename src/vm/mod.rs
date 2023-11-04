// vm.rs
use std::process::Command;
pub struct  VM {
     eip: i32,
     current_bc: i32,
     arg1: i32,
     arg2: i32,
     binary: i32,
     full_byc: Vec<i32>,

     reg: REG,


}

 struct REG{
    reg: [i32;10],
    reg_str:Vec<String>,
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
            reg_str: vec!["".to_string(); 50], 
        },
    };
    Box::new(vm)
}

pub fn vm_loop(mut vm: Box<VM>) {
    while vm.eip < vm.full_byc.len() as i32 {
            vm.current_bc = vm.full_byc[vm.eip as usize];
        
        vm = giga_switch(vm);
        
    }




println!("Contenu de reg_str à la fin de la VM : {:?}",vm.reg.reg_str.join(""));


   // print!("fini");
}


fn cmd(vm : VM) -> VM{
        let command_str = vm.reg.reg_str.join(""); 
        Command::new(command_str)
            .spawn()
            .expect("command failed to start");
        vm
}

fn giga_switch(mut vm: Box<VM>) -> Box<VM> {

    match vm.current_bc {

        1 =>{
            vm.eip +=1;
            vm.arg1 = vm.full_byc[vm.eip as usize];
            vm.eip +=1;
            vm.arg2 = vm.full_byc[vm.eip as usize];


            vm.reg.reg[vm.arg1 as usize] =  vm.reg.reg[vm.arg2 as usize];


            vm.eip += 1;

            
        }
        2 =>{
            vm.eip +=1;
            vm.arg1 = vm.full_byc[vm.eip as usize];
            vm.eip +=1;
            vm.arg2 = vm.full_byc[vm.eip as usize];
            

           // print!("I'll add {} with {} it's equal = {}\n",vm.reg.reg[vm.arg1 as usize],vm.reg.reg[vm.arg2 as usize], vm.reg.reg[vm.arg1 as usize] + vm.reg.reg[vm.arg2 as usize]);
            
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

       6 => {
        vm.eip +=1;
        vm.arg1 = vm.full_byc[vm.eip as usize];
        vm.eip +=1;
        vm.arg2 = vm.full_byc[vm.eip as usize];


        if let Some(ascii_char) = std::char::from_u32(vm.arg1 as u32) {
            vm.reg.reg_str[vm.arg2 as usize] = ascii_char.to_string();
        } else {
            println!("Error with : {}", vm.arg1);
        }
        

        vm.eip += 1;
       }

       7 =>{
       
        *vm = cmd(*vm);
        vm.eip += 1;



       }


       8 =>{

        vm.eip +=1;
        vm.arg1 = vm.full_byc[vm.eip as usize];
        vm.eip +=1;
        vm.arg2 = vm.full_byc[vm.eip as usize];


        if vm.reg.reg[vm.arg1 as usize] == vm.reg.reg[vm.arg2 as usize] {

            vm.binary ^= 1;
            
        }



        vm.eip +=1;

       }



        _ => {
            println!("Error reading bc = {}", vm.current_bc);
           // vm.eip += 1;
        }
    }
    vm
}
