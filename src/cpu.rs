use std::collections::HashMap;
mod bus;
use bus::{RAM};




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
 }
//check A


 pub fn LOAD_X(regs : &mut Self, num:u8){
    regs.X =num;
}
//change X
 pub fn CHECK_X(regs:&Self)->u8{
  regs.X
}
//check X

 pub fn LOAD_Y(regs: &mut Self, num:u8){
    regs.Y= num;
}
//change Y
 pub fn CHECK_Y(regs :&Self)->u8{
  regs.Y

}
//check Y

 pub fn LOAD_Sp(regs : &mut Self, num:u8){
    regs.Sp =num; 
}
//change Sp
 pub fn CHECK_Sp(regs : &Self)->u16{
    0x0100 + regs.Sp as u16
}
//check Sp // stack pointer points to 0X0100 -0x01ff path

 pub fn LOAD_Pc(regs: &mut Self, num:u16){
    regs.Pc = num;
}
//change A
 pub fn CHECK_Pc(regs :&Self)->u16{
    regs.Pc
}
//check A


 pub fn SET_FLAG(regs:&mut Self, flag:Flags){
   match flag{
     Flags::C=>{regs.Str[0]=true;},
     Flags::Z=>{regs.Str[1]=true;},
     Flags::I=>{regs.Str[2]=true;},
     Flags::D=>{regs.Str[3]=true;},
     Flags::B=>{regs.Str[4]=true;},
     Flags::U=>{regs.Str[5]=false;},
     Flags::V=>{regs.Str[6]=true;},
     Flags::N=>{regs.Str[7]=true;}  }
    }
//set flag in str

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
  }
//check flag
  }



pub struct Instruction{
 pub addr_mode : fn(&mut RAM, &mut Regs  ) ->u16,
 pub instruct : fn(&mut RAM, &mut Regs  ) ->u16,
 pub cycle : u8
 }

pub struct oper_var{
  abs_addr:u16,
  rel_addr:u8,
  curr_data:u8
  
 }
//--------------------------------------------------------------------------------------------------
//functions for addressing modes
  pub fn IMM(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
      cpu_reg.Pc +=1;
      oper.curr_data = RAM::read(memo, cpu_reg.Pc);
      oper.abs_addr= cpu_reg.Pc;
      cpu_reg.Pc +=1;
      false
    } 
  pub fn ABS(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
     cpu_reg.Pc +=1;
     oper.abs_addr = RAM::read(memo ,cpu_reg.Pc) as u16 + (RAM::read(memo ,cpu_reg.Pc +1) as u16 ) *0x0100;
     cpu_reg.Pc +=2;
     oper.curr_data =RAM::read(memo ,oper.abs_addr);
     false



   }
  pub fn ABX(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
      cpu_reg.Pc +=1;
      oper.abs_addr = RAM::read(memo, cpu_reg.Pc) as u16 +(RAM::read(memo, cpu_reg.Pc +1) as u16)*0x0100;
      oper.abs_addr += cpu_reg.X as u16;
      oper.curr_data = RAM::read(memo ,oper.abs_addr);
      cpu_reg.Pc +=2;
      if (oper.abs_addr &0xff00) != ((RAM::read(memo, cpu_reg.Pc -1) as u16)*0x0100){
        true
      }else
      {false}

    }
  pub fn ABY(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
    cpu_reg.Pc +=1;
      oper.abs_addr = RAM::read(memo, cpu_reg.Pc) as u16 +(RAM::read(memo, cpu_reg.Pc +1) as u16)*0x0100;
      oper.abs_addr += cpu_reg.Y as u16;
      oper.curr_data = RAM::read(memo ,oper.abs_addr);
      cpu_reg.Pc +=2;
      if (oper.abs_addr &0xff00) != ((RAM::read(memo, cpu_reg.Pc -1) as u16)*0x0100){
        true
      }else
      {false}
    } 
  pub fn ZP0(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
    cpu_reg.Pc +=1;
    oper.abs_addr = RAM::read(memo ,cpu_reg.Pc) as u16 ;
    oper.curr_data = RAM::read(memo ,oper.abs_addr);
    cpu_reg.Pc +=1;
      false

    } 
  pub fn ZPX(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
      cpu_reg.Pc +=1;
      oper.abs_addr = RAM::read(memo , cpu_reg.Pc) as u16 ;
      oper.abs_addr += cpu_reg.X as u16;
      oper.abs_addr &= 0x00ff;
      oper.curr_data = RAM::read(memo , oper.abs_addr);
      cpu_reg.Pc +=1;
      false

   } 
  pub fn ZPY(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
    cpu_reg.Pc +=1;
    oper.abs_addr = RAM::read(memo , cpu_reg.Pc) as u16 ;
    oper.abs_addr += cpu_reg.Y as u16;
    oper.abs_addr &= 0x00ff;
    oper.curr_data = RAM::read(memo , oper.abs_addr);
    cpu_reg.Pc +=1;
    false
    } 
  pub fn REL(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
    cpu_reg.Pc +=1;
    oper.rel_addr =RAM::read(memo ,cpu_reg.Pc);
    if oper.rel_addr>>0x07 != 1
    {oper.abs_addr = cpu_reg.Pc + oper.rel_addr as u16;}else
     {oper.abs_addr = cpu_reg.Pc + (oper.rel_addr as u16 ^0xffff)};
    cpu_reg.Pc = oper.abs_addr;
    false

   } 
  pub fn IMP(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
    oper.curr_data = cpu_reg.A;
    oper.abs_addr = cpu_reg.Pc;
    cpu_reg.Pc +=1;
    false
    } 
  pub fn IND(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
    cpu_reg.Pc +=1;
    if (cpu_reg.Pc & 0x00ff != 0x00ff){
    oper.abs_addr = RAM::read(memo , cpu_reg.Pc) as u16  +(RAM::read(memo , cpu_reg.Pc +1) as u16)* 0x0100;
    }else{ oper.abs_addr = RAM::read(memo , cpu_reg.Pc) as u16  +(RAM::read(memo , cpu_reg.Pc &0xff00) as u16)* 0x0100;}
    oper.abs_addr = RAM::read(memo , oper.abs_addr) as u16  +(RAM::read(memo , oper.abs_addr +1) as u16)* 0x0100; 
    oper.curr_data = RAM ::read(memo , oper.abs_addr) ;
    cpu_reg.Pc +=2;
    false

    } 
  pub fn ZIX(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
    cpu_reg.Pc +=1;
    oper.rel_addr = RAM::read(memo  , cpu_reg.Pc) + cpu_reg.X;
    oper.abs_addr= RAM ::read(memo , oper.rel_addr as u16) as u16 +(RAM ::read(memo ,oper.rel_addr as u16 +1) as u16)*0x0100;
    oper.curr_data =RAM ::read (memo , oper.abs_addr);
    cpu_reg.A = oper.curr_data;
    cpu_reg.Pc +=1;
    false

   } 
  pub fn ZIY(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
      cpu_reg.Pc +=1;
      oper.abs_addr= RAM::read(memo ,cpu_reg.Pc) as u16 ;
      oper.abs_addr =  RAM::read(memo , oper.abs_addr) as u16+(RAM::read(memo , oper.abs_addr+1 ) as u16)*0x0100 + cpu_reg.Y as u16;
      cpu_reg.A = RAM ::read(memo ,oper.abs_addr);
      cpu_reg.Pc +=1;
      false

   }
  pub fn ACC(memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
    cpu_reg.Pc +=1;
    oper.curr_data = cpu_reg.A;
    oper.abs_addr = cpu_reg.Pc;
    cpu_reg.Pc +=1;
    false
    }




//44444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444
       //   pub fn  ADC (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
        let mut data : u16 = cpu_reg.A as u16  + Regs::GET_FLAG(cpu_reg, Flags::C) as u16 *0x0100 + oper.curr_data as u16;
        if data &0x0080 !=0{Regs::SET_FLAG(cpu_reg , Flags::N)};
        if data & 0x00ff ==0{Regs::SET_FLAG(cpu_reg, Flags::Z)} ;
        if(!(cpu_reg.A as u16 ^oper.curr_data as u16) &(cpu_reg.A as u16 ^ data))&0x80>0{Regs::SET_FLAG(cpu_reg, Flags::V)};
        if(data >= 256){Regs::SET_FLAG(cpu_reg, Flags::C)};
        cpu_reg.A = (data & 0x00ff ) as u8;
        true


         } 

      pub fn  AND (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
          cpu_reg.A &= oper.curr_data;
          if(cpu_reg.A & 0x80)!=0{Regs::SET_FLAG(cpu_reg , Flags::N)};
          if(cpu_reg.A &0xff)==0{Regs::SET_FLAG(cpu_reg ,Flags::Z)};
          true
       }
      pub fn  ASL (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
         cpu_reg.A =oper.curr_data;
         if(cpu_reg.A &0x80 != 0) {Regs::SET_FLAG(cpu_reg , Flags::C)};
         cpu_reg.A <<=1;
         if(cpu_reg.A &0x80 != 0) {Regs::SET_FLAG(cpu_reg, Flags::N)};
         if(cpu_reg.A ==0){Regs::SET_FLAG(cpu_reg, Flags::Z)};
         if(oper.abs_addr !=cpu_reg.Pc-1){
          RAM::write(memo , cpu_reg.A, oper.abs_addr);
         }
         false

       }
      pub fn  BCC (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
          if(!Regs::GET_FLAG(cpu_reg , Flags::C)){cpu_reg.Pc += oper.rel_addr;
          true}else{false}



        }
      pub fn  BCS (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
        if(Regs::GET_FLAG(cpu_reg , Flags::C)){cpu_reg.Pc += oper.rel_addr;
          true}else{false}
         }
      pub fn  BEQ (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->bool{
        if(Regs::GET_FLAG(cpu_reg,Flags::Z)){cpu_reg.abs_addr += oper.rel_addr;true}
        false

        }
      pub fn  BIT (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  BMI (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  BNE (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  BPL (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  BRK (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  BVC (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  BVS (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  CLC (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  CLD (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  CLI (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  CLV (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  CMP (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  CPX (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  CPY (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  DEC (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  DEX (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  DEY (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  EOR (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  INC (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  INX (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  INY (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  JMP (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  JSR (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  LDA (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  LDX (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  LDY (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  LSR (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  NOP (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  ORA (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  PHA (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  PHP (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  PLA (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  PLP (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  ROL (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  ROR (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  RTI (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  RTS (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  SBC (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  SEC (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  SED (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  SEI (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  STA (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  STX (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  STY (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  TAX (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  TAY (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  TSX (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  TXA (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  TXS (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}
      pub fn  TYA (memo: &mut RAM, cpu_reg:&mut Regs, oper: &mut oper_var)->(){}


//------------------------------------------------------------------------------------------------------------
pub enum A_mode{

  IMM, 
  ABS,
  ABX,
  ABY, 
  ZP0, 
  ZPX, 
  ZPY, 
  REL, 
  IMP, 
  IND, 
  IDX, 
  IDY,
  ACC

}
// pub enum Instruct{

//   ADC ,AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE BPL BRK BVC BVS CLC
//   CLD, CLI, CLV, CMP, CPX, CPY, DEC, DEX, DEY EOR INC INX INY JMP
//   JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP PLA PLP ROL ROR RTI
//   RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX TAY TSX TXA TXS TYA
// }


// pub static Hex_instruct:HashMap<u8,>












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


fn main (){




}






















// mod bus;
// use bus::{memoram as mem,RAM};


// fn main(){

//     unsafe{
//         RAM::write(&mut mem ,32,0x0001);
//     println!("{}",mem.ram[1]);}
// }
