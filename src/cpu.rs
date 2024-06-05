pub enum Flags{
C,
Z,
I,
D,
B,
U,
V,
N

}

pub struct Regs{
pub A:u8,
pub X:u8,
pub Y:u8,
pub Sp:u8,
pub Pc:u16,
pub Str:[bool;8]}

impl  Regs{
pub fn LOAD_A(regs :&mut Self, num:u8){
     regs.A = num;
}
//change A
pub fn CHECK_A(regs: &Self)->u8{
  regs.A
}//check A

pub fn LOAD_X(regs : &mut Self, num:u8){
    regs.X =num;
}//change X
pub fn CHECK_X(regs:&Self)->u8{
  regs.X
}//check X

pub fn LOAD_Y(regs: &mut Self, num:u8){
    regs.Y= num;
}//change Y
pub fn CHECK_Y(regs :&Self)->u8{
  regs.Y

}//check Y

pub fn LOAD_Sp(regs : &mut Self, num:u8){
    regs.Sp =num; 
}//change Sp
pub fn CHECK_Sp(regs : &Self)->u16{
    0x0100 + regs.Sp as u16
}//check Sp // stack pointer points to 0X0100 -0x01ff path

pub fn LOAD_Pc(regs: &mut Self, num:u16){
    regs.Pc = num;
}//change A
pub fn CHECK_Pc(regs :&Self)->u16{
    regs.Pc
}//check A


pub fn SET_FLAG(regs:&mut Self, flag:Flags){
   match flag{
     Flags::C=>{regs.Str[0]=true;},
     Flags::Z=>{regs.Str[1]=true;},
     Flags::I=>{regs.Str[2]=true;},
     Flags::D=>{regs.Str[3]=true;},
     Flags::B=>{regs.Str[4]=true;},
     Flags::U=>{regs.Str[5]=false;},
     Flags::V=>{regs.Str[6]=true;},
     Flags::N=>{regs.Str[7]=true;}





   }


}//set flag in str
pub fn GET_FLAG(regs:&Self,flag:Flags)->bool{


   let var:bool= match flag{
        Flags::C=>{regs.Str[0]},
        Flags::Z=>{regs.Str[1]},
        Flags::I=>{regs.Str[2]},
        Flags::D=>{regs.Str[3]},
        Flags::B=>{regs.Str[4]},
        Flags::U=>{regs.Str[5]},
        Flags::V=>{regs.Str[6]},
        Flags::N=>{regs.Str[7]}
   
   
   
   
   
      };
      var

}//check flag




}












//opcodes
// {

//     ADC AND ASL BCC BCS BEQ BIT BMI BNE BPL BRK BVC BVS CLC
//     CLD CLI CLV CMP CPX CPY DEC DEX DEY EOR INC INX INY JMP
//     JSR LDA LDX LDY LSR NOP ORA PHA PHP PLA PLP ROL ROR RTI
//     RTS SBC SEC SED SEI STA STX STY TAX TAY TSX TXA TXS TYA

// }
//addressing mode
// {
//  IMP ACC IMM ABS PCR STK ZPG
//  ABX ABY ZPX ZPY ABI ZIX ZIY 

    
// }





// GET_FLAG()


fn main (){}






















// mod bus;
// use bus::{memoram as mem,RAM};


// fn main(){

//     unsafe{
//         RAM::write(&mut mem ,32,0x0001);
//     println!("{}",mem.ram[1]);}
// }
