// commadn Buatan untuk shell M Shell ini
//
//format : m->vlue1->value2->....

pub struct MShell<'a>{
    command : &'a mut String,
}


impl<'a> MShell <'a>{
    
    // Pisahkan dan UPdate command Commadn 
    // fn splite_command(&self)->Vec<&str>{
    //
    //     // splite dengan tanda panah
    //     let str : &Vec<&str>= &self.command.split("->").collect();
    //     str
    //
    // }
}
