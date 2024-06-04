enum Flags{
C =1 <<0,
Z =1 <<1,
I =1 <<2,
D =1 << 3,
B =1 <<4,
V =1 <<5,
N =1 <<6,
U =1 <<7

}

pub struct Regs{
pub A:u8,
pub X:u8,
pub Y:u8,
pub Sp:u8,
pub Pc:u16,
pub Str:[8;Flags]}


//opcodes
{

    ADC AND ASL BCC BCS BEQ BIT BMI BNE BPL BRK BVC BVS CLC
    CLD CLI CLV CMP CPX CPY DEC DEX DEY EOR INC INX INY JMP
    JSR LDA LDX LDY LSR NOP ORA PHA PHP PLA PLP ROL ROR RTI
    RTS SBC SEC SED SEI STA STX STY TAX TAY TSX TXA TXS TYA

}
//addressing mode
fn main (){}






















// mod bus;
// use bus::{memoram as mem,RAM};


// fn main(){

//     unsafe{
//         RAM::write(&mut mem ,32,0x0001);
//     println!("{}",mem.ram[1]);}
// }
