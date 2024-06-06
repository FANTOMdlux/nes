pub trait manipulate{
     fn read(&Self,u16)->(){}

     fn write(&mut Self, u8,u16)->(){}
  }

///
pub struct RAM{ pub ram:[u8;1024*64]}

pub static mut memoram :RAM =RAM{ram:[0;1024*64 ]};

impl RAM{

  pub  fn read(s:&Self,ram_addr:u16)->u8{
    let mut name : u8 =0;
    
    if ram_addr>=0x0000 &&ram_addr<=0xFFFF
      {  *&mut name =  s.ram[ram_addr as usize];} 

      name

    }

  pub fn write(none:&mut Self,data: u8,ram_addr:u16)->() {

    

    if(ram_addr>=0x0000 &&ram_addr<=0xFFFF)
        { none.ram[ram_addr as usize]=data;}
 

   }




 }
 fn main(){}