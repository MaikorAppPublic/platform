pub const NOP: u8 = 0x00;
pub const HALT: u8 = 0x01;
pub const CALL_ADDR: u8 = 0x02;
pub const CALL_REG: u8 = 0x03;
pub const RET: u8 = 0x04;
pub const RETI: u8 = 0x05;
pub const SWAP_REG_REG_BYTE: u8 = 0x06;
pub const SWAP_REG_REG_WORD: u8 = 0x07;
pub const MEM_CPY_ADDR_ADDR_BYTE: u8 = 0x08;
pub const MEM_CPY_ADDR_REG_BYTE: u8 = 0x09;
pub const MEM_CPY_REG_ADDR_BYTE: u8 = 0x0A;
pub const MEM_CPY_REG_REG_BYTE: u8 = 0x0B;
pub const MEM_CPY_ADDR_ADDR_REG: u8 = 0x0C;
pub const MEM_CPY_ADDR_REG_REG: u8 = 0x0D;
pub const MEM_CPY_REG_ADDR_REG: u8 = 0x0E;
pub const MEM_CPY_REG_REG_REG: u8 = 0x0F;

pub const CPY_REG_REG_BYTE: u8 = 0x10;
pub const CPY_REG_REG_WORD: u8 = 0x11;
pub const CPY_ADDR_REG_BYTE: u8 = 0x12;
pub const CPY_ADDR_REG_WORD: u8 = 0x13;
pub const CPY_REG_ADDR_BYTE: u8 = 0x14;
pub const CPY_REG_ADDR_WORD: u8 = 0x15;
pub const CPY_ADDR_ADDR_BYTE: u8 = 0x16;
pub const CPY_ADDR_ADDR_WORD: u8 = 0x17;
pub const CPY_REG_NUM_BYTE: u8 = 0x18;
pub const CPY_REG_NUM_WORD: u8 = 0x19;
pub const CPY_ADDR_NUM_BYTE: u8 = 0x1A;
pub const CPY_ADDR_NUM_WORD: u8 = 0x1B;
pub const CMP_REG_ADDR_BYTE: u8 = 0x1C;
pub const CMP_REG_ADDR_WORD: u8 = 0x1D;
pub const CMPS_REG_ADDR_BYTE: u8 = 0x1E;
pub const CMPS_REG_ADDR_WORD: u8 = 0x1F;

pub const ADD_REG_REG_BYTE: u8 = 0x20;
pub const ADD_REG_REG_WORD: u8 = 0x21;
pub const ADD_REG_NUM_BYTE: u8 = 0x22;
pub const ADD_REG_NUM_WORD: u8 = 0x23;
pub const ADD_REG_ADDR_BYTE: u8 = 0x24;
pub const ADD_REG_ADDR_WORD: u8 = 0x25;
pub const ADD_ADDR_REG_BYTE: u8 = 0x26;
pub const ADD_ADDR_REG_WORD: u8 = 0x27;
pub const ADD_ADDR_NUM_BYTE: u8 = 0x28;
pub const ADD_ADDR_NUM_WORD: u8 = 0x29;
pub const ADD_ADDR_ADDR_BYTE: u8 = 0x2A;
pub const ADD_ADDR_ADDR_WORD: u8 = 0x2B;
pub const INC_REG_BYTE: u8 = 0x2C;
pub const INC_REG_WORD: u8 = 0x2D;
pub const INC_ADDR_BYTE: u8 = 0x2E;
pub const INC_ADDR_WORD: u8 = 0x2F;

pub const SUB_REG_REG_BYTE: u8 = 0x30;
pub const SUB_REG_REG_WORD: u8 = 0x31;
pub const SUB_REG_NUM_BYTE: u8 = 0x32;
pub const SUB_REG_NUM_WORD: u8 = 0x33;
pub const SUB_REG_ADDR_BYTE: u8 = 0x34;
pub const SUB_REG_ADDR_WORD: u8 = 0x35;
pub const SUB_ADDR_REG_BYTE: u8 = 0x36;
pub const SUB_ADDR_REG_WORD: u8 = 0x37;
pub const SUB_ADDR_NUM_BYTE: u8 = 0x38;
pub const SUB_ADDR_NUM_WORD: u8 = 0x39;
pub const SUB_ADDR_ADDR_BYTE: u8 = 0x3A;
pub const SUB_ADDR_ADDR_WORD: u8 = 0x3B;
pub const DEC_REG_BYTE: u8 = 0x3C;
pub const DEC_REG_WORD: u8 = 0x3D;
pub const DEC_ADDR_BYTE: u8 = 0x3E;
pub const DEC_ADDR_WORD: u8 = 0x3F;

pub const NOT_REG_BYTE: u8 = 0x40;
pub const NOT_REG_WORD: u8 = 0x41;
pub const OR_REG_REG_BYTE: u8 = 0x42;
pub const OR_REG_REG_WORD: u8 = 0x43;
pub const OR_REG_NUM_BYTE: u8 = 0x44;
pub const OR_REG_NUM_WORD: u8 = 0x45;
pub const XOR_REG_REG_BYTE: u8 = 0x46;
pub const XOR_REG_REG_WORD: u8 = 0x47;
pub const XOR_REG_NUM_BYTE: u8 = 0x48;
pub const XOR_REG_NUM_WORD: u8 = 0x49;
pub const AND_REG_REG_BYTE: u8 = 0x4A;
pub const AND_REG_REG_WORD: u8 = 0x4B;
pub const AND_REG_NUM_BYTE: u8 = 0x4C;
pub const AND_REG_NUM_WORD: u8 = 0x4D;
pub const JRF_BYTE: u8 = 0x4E;
pub const JRB_BYTE: u8 = 0x4F;

pub const ASL_REG_NUM_BYTE: u8 = 0x50;
pub const ASL_REG_NUM_WORD: u8 = 0x51;
pub const ASL_REG_REG_BYTE: u8 = 0x52;
pub const ASL_REG_REG_WORD: u8 = 0x53;
pub const ASL_ADDR_BYTE: u8 = 0x54;
pub const ASL_ADDR_WORD: u8 = 0x55;
pub const ASR_REG_NUM_BYTE: u8 = 0x56;
pub const ASR_REG_NUM_WORD: u8 = 0x57;
pub const ASR_REG_REG_BYTE: u8 = 0x58;
pub const ASR_REG_REG_WORD: u8 = 0x59;
pub const ASR_ADDR_BYTE: u8 = 0x5A;
pub const ASR_ADDR_WORD: u8 = 0x5B;
pub const LSR_REG_NUM_BYTE: u8 = 0x5C;
pub const LSR_REG_NUM_WORD: u8 = 0x5D;
pub const LSR_REG_REG_BYTE: u8 = 0x5E;
pub const LSR_REG_REG_WORD: u8 = 0x5F;
pub const LSR_ADDR_BYTE: u8 = 0x60;
pub const LSR_ADDR_WORD: u8 = 0x61;
pub const ROL_REG_NUM_BYTE: u8 = 0x62;
pub const ROL_REG_NUM_WORD: u8 = 0x63;
pub const ROL_REG_REG_BYTE: u8 = 0x64;
pub const ROL_REG_REG_WORD: u8 = 0x65;
pub const ROL_ADDR_BYTE: u8 = 0x66;
pub const ROL_ADDR_WORD: u8 = 0x67;
pub const ROR_REG_NUM_BYTE: u8 = 0x68;
pub const ROR_REG_NUM_WORD: u8 = 0x69;
pub const ROR_REG_REG_BYTE: u8 = 0x6A;
pub const ROR_REG_REG_WORD: u8 = 0x6B;
pub const ROR_ADDR_BYTE: u8 = 0x6C;
pub const ROR_ADDR_WORD: u8 = 0x6D;
//unused 2 0x6E - 0x6F

pub const PUSH_REG_BYTE: u8 = 0x70;
pub const PUSH_REG_WORD: u8 = 0x72;
pub const PUSH_NUM_BYTE: u8 = 0x71;
pub const PUSH_NUM_WORD: u8 = 0x73;
pub const POP_REG_BYTE: u8 = 0x74;
pub const POP_REG_WORD: u8 = 0x75;
//unused 10 0x76 - 0x7F

pub const MUL_REG_REG_BYTE: u8 = 0x80;
pub const MUL_REG_REG_WORD: u8 = 0x81;
pub const MUL_REG_NUM_BYTE: u8 = 0x82;
pub const MUL_REG_NUM_WORD: u8 = 0x83;
pub const MUL_REG_ADDR_BYTE: u8 = 0x84;
pub const MUL_REG_ADDR_WORD: u8 = 0x85;
pub const MUL_ADDR_REG_BYTE: u8 = 0x86;
pub const MUL_ADDR_REG_WORD: u8 = 0x87;
pub const MUL_ADDR_NUM_BYTE: u8 = 0x88;
pub const MUL_ADDR_NUM_WORD: u8 = 0x89;
pub const MUL_ADDR_ADDR_BYTE: u8 = 0x8A;
pub const MUL_ADDR_ADDR_WORD: u8 = 0x8B;
pub const MULS_REG_REG_BYTE: u8 = 0x8C;
pub const MULS_REG_REG_WORD: u8 = 0x8D;
pub const MULS_REG_NUM_BYTE: u8 = 0x8E;
pub const MULS_REG_NUM_WORD: u8 = 0x8F;
pub const MULS_REG_ADDR_BYTE: u8 = 0x90;
pub const MULS_REG_ADDR_WORD: u8 = 0x91;
pub const MULS_ADDR_REG_BYTE: u8 = 0x92;
pub const MULS_ADDR_REG_WORD: u8 = 0x93;
pub const MULS_ADDR_NUM_BYTE: u8 = 0x94;
pub const MULS_ADDR_NUM_WORD: u8 = 0x95;
pub const MULS_ADDR_ADDR_BYTE: u8 = 0x96;
pub const MULS_ADDR_ADDR_WORD: u8 = 0x97;
pub const DIV_REG_REG_BYTE: u8 = 0x98;
pub const DIV_REG_REG_WORD: u8 = 0x99;
pub const DIV_REG_NUM_BYTE: u8 = 0x9A;
pub const DIV_REG_NUM_WORD: u8 = 0x9B;
pub const DIV_REG_ADDR_BYTE: u8 = 0x9C;
pub const DIV_REG_ADDR_WORD: u8 = 0x9D;
pub const DIV_ADDR_REG_BYTE: u8 = 0x9E;
pub const DIV_ADDR_REG_WORD: u8 = 0x9F;
pub const DIV_ADDR_NUM_BYTE: u8 = 0xA0;
pub const DIV_ADDR_NUM_WORD: u8 = 0xA1;
pub const DIV_ADDR_ADDR_BYTE: u8 = 0xA2;
pub const DIV_ADDR_ADDR_WORD: u8 = 0xA3;
pub const DIVS_REG_REG_BYTE: u8 = 0xA4;
pub const DIVS_REG_REG_WORD: u8 = 0xA5;
pub const DIVS_REG_NUM_BYTE: u8 = 0xA6;
pub const DIVS_REG_NUM_WORD: u8 = 0xA7;
pub const DIVS_REG_ADDR_BYTE: u8 = 0xA8;
pub const DIVS_REG_ADDR_WORD: u8 = 0xA9;
pub const DIVS_ADDR_REG_BYTE: u8 = 0xAA;
pub const DIVS_ADDR_REG_WORD: u8 = 0xAB;
pub const DIVS_ADDR_NUM_BYTE: u8 = 0xAC;
pub const DIVS_ADDR_NUM_WORD: u8 = 0xAD;
pub const DIVS_ADDR_ADDR_BYTE: u8 = 0xAE;
pub const DIVS_ADDR_ADDR_WORD: u8 = 0xAF;

pub const JMP_ADDR: u8 = 0xB0;
pub const JMP_REG: u8 = 0xB1;
pub const JE_ADDR: u8 = 0xB2;
pub const JE_REG: u8 = 0xB3;
pub const JNE_ADDR: u8 = 0xB4;
pub const JNE_REG: u8 = 0xB5;
pub const JL_ADDR: u8 = 0xB6;
pub const JL_REG: u8 = 0xB7;
pub const JG_ADDR: u8 = 0xB8;
pub const JG_REG: u8 = 0xB9;
pub const JLE_ADDR: u8 = 0xBA;
pub const JLE_REG: u8 = 0xBB;
pub const JGE_ADDR: u8 = 0xBC;
pub const JGE_REG: u8 = 0xBD;
//unused 2 0xBE - 0xBF

pub const CMP_REG_NUM_BYTE: u8 = 0xC0;
pub const CMP_REG_NUM_WORD: u8 = 0xC1;
pub const CMP_REG_REG_BYTE: u8 = 0xC2;
pub const CMP_REG_REG_WORD: u8 = 0xC3;
pub const CMPS_REG_NUM_BYTE: u8 = 0xC4;
pub const CMPS_REG_NUM_WORD: u8 = 0xC5;
pub const CMPS_REG_REG_BYTE: u8 = 0xC6;
pub const CMPS_REG_REG_WORD: u8 = 0xC7;
pub const JBC_REG_REG: u8 = 0xC8;
pub const JBS_REG_REG: u8 = 0xC9;
pub const JBC_ADDR_REG: u8 = 0xCA;
pub const JBS_ADDR_REG: u8 = 0xCB;
pub const JBC_REG_NUM: u8 = 0xCC;
pub const JBS_REG_NUM: u8 = 0xCD;
pub const JBC_ADDR_NUM: u8 = 0xCE;
pub const JBS_ADDR_NUM: u8 = 0xCF;
//unused 16 0xD0 - 0xDF

pub const ADDC_REG_REG_BYTE: u8 = 0xE0;
pub const ADDC_REG_REG_WORD: u8 = 0xE1;
pub const ADDC_REG_NUM_BYTE: u8 = 0xE2;
pub const ADDC_REG_NUM_WORD: u8 = 0xE3;
pub const ADDC_REG_ADDR_BYTE: u8 = 0xE4;
pub const ADDC_REG_ADDR_WORD: u8 = 0xE5;
pub const ADDC_ADDR_REG_BYTE: u8 = 0xE6;
pub const ADDC_ADDR_REG_WORD: u8 = 0xE7;
pub const ADDC_ADDR_NUM_BYTE: u8 = 0xE8;
pub const ADDC_ADDR_NUM_WORD: u8 = 0xE9;
pub const ADDC_ADDR_ADDR_BYTE: u8 = 0xEA;
pub const ADDC_ADDR_ADDR_WORD: u8 = 0xEB;
pub const SUBC_REG_REG_BYTE: u8 = 0xEC;
pub const SUBC_REG_REG_WORD: u8 = 0xED;
pub const SUBC_REG_NUM_BYTE: u8 = 0xEE;
pub const SUBC_REG_NUM_WORD: u8 = 0xEF;
pub const SUBC_REG_ADDR_BYTE: u8 = 0xF0;
pub const SUBC_REG_ADDR_WORD: u8 = 0xF1;
pub const SUBC_ADDR_REG_BYTE: u8 = 0xF2;
pub const SUBC_ADDR_REG_WORD: u8 = 0xF3;
pub const SUBC_ADDR_NUM_BYTE: u8 = 0xF4;
pub const SUBC_ADDR_NUM_WORD: u8 = 0xF5;
pub const SUBC_ADDR_ADDR_BYTE: u8 = 0xF6;
pub const SUBC_ADDR_ADDR_WORD: u8 = 0xF7;
//unused 8 0xF8 - 0xFD

pub const EHALT: u8 = 0xFE;
pub const SLEEP: u8 = 0xFF;

///List of ops that MUST affect PC
/// This means that they unconditionally set PC
/// however they might set to it's current value
/// (e.g. if the whole program was 'JMP $0')
pub const MUST_JMP_OPS: [u8; 8] = [
    JMP_REG, JMP_ADDR, CALL_ADDR, CALL_REG, RET, RETI, JRF_BYTE, JRB_BYTE,
];

///List of ops that MAY affect PC
pub const MAY_JMP_OPS: [u8; 20] = [
    JL_REG,
    JL_ADDR,
    JLE_REG,
    JLE_ADDR,
    JG_REG,
    JG_ADDR,
    JGE_REG,
    JGE_ADDR,
    JE_REG,
    JE_ADDR,
    JNE_REG,
    JNE_ADDR,
    JBS_REG_REG,
    JBS_REG_NUM,
    JBS_ADDR_REG,
    JBS_ADDR_NUM,
    JBC_REG_REG,
    JBC_REG_NUM,
    JBC_ADDR_REG,
    JBC_ADDR_NUM,
];

#[rustfmt::skip]
pub const ALL: [u8; 220] = [
    NOP, HALT,
    CALL_ADDR, CALL_REG, RET, RETI,
    CPY_REG_REG_BYTE, CPY_REG_REG_WORD, CPY_ADDR_REG_BYTE, CPY_ADDR_REG_WORD,
    CPY_REG_ADDR_BYTE, CPY_REG_ADDR_WORD, CPY_ADDR_ADDR_BYTE, CPY_ADDR_ADDR_WORD,
    CPY_REG_NUM_BYTE, CPY_REG_NUM_WORD, CPY_ADDR_NUM_BYTE, CPY_ADDR_NUM_WORD,
    ADD_REG_REG_BYTE, ADD_REG_REG_WORD, ADD_REG_NUM_BYTE, ADD_REG_NUM_WORD,
    ADD_REG_ADDR_BYTE, ADD_REG_ADDR_WORD, ADD_ADDR_REG_BYTE, ADD_ADDR_REG_WORD,
    ADD_ADDR_NUM_BYTE, ADD_ADDR_NUM_WORD, ADD_ADDR_ADDR_BYTE, ADD_ADDR_ADDR_WORD,
    INC_REG_BYTE, INC_REG_WORD, INC_ADDR_BYTE, INC_ADDR_WORD,
    SUB_REG_REG_BYTE, SUB_REG_REG_WORD, SUB_REG_NUM_BYTE, SUB_REG_NUM_WORD,
    SUB_REG_ADDR_BYTE, SUB_REG_ADDR_WORD, SUB_ADDR_REG_BYTE, SUB_ADDR_REG_WORD,
    SUB_ADDR_NUM_BYTE, SUB_ADDR_NUM_WORD, SUB_ADDR_ADDR_BYTE, SUB_ADDR_ADDR_WORD,
    DEC_REG_BYTE, DEC_REG_WORD, DEC_ADDR_BYTE, DEC_ADDR_WORD,
    SWAP_REG_REG_BYTE, SWAP_REG_REG_WORD,
    MEM_CPY_ADDR_ADDR_BYTE, MEM_CPY_ADDR_REG_BYTE, MEM_CPY_REG_ADDR_BYTE, MEM_CPY_REG_REG_BYTE,
    MEM_CPY_ADDR_ADDR_REG, MEM_CPY_ADDR_REG_REG, MEM_CPY_REG_ADDR_REG, MEM_CPY_REG_REG_REG,
    NOT_REG_BYTE, NOT_REG_WORD,
    OR_REG_REG_BYTE, OR_REG_REG_WORD, OR_REG_NUM_BYTE, OR_REG_NUM_WORD,
    XOR_REG_REG_BYTE, XOR_REG_REG_WORD, XOR_REG_NUM_BYTE, XOR_REG_NUM_WORD,
    AND_REG_REG_BYTE, AND_REG_REG_WORD, AND_REG_NUM_BYTE, AND_REG_NUM_WORD,
    ASL_REG_NUM_BYTE, ASL_REG_REG_BYTE, ASL_REG_NUM_WORD,
    ASL_REG_REG_WORD, ASL_ADDR_BYTE, ASL_ADDR_WORD,
    ASR_REG_NUM_BYTE, ASR_REG_REG_BYTE, ASR_REG_NUM_WORD,
    ASR_REG_REG_WORD, ASR_ADDR_BYTE, ASR_ADDR_WORD,
    LSR_REG_NUM_BYTE, LSR_REG_REG_BYTE, LSR_REG_NUM_WORD,
    LSR_REG_REG_WORD, LSR_ADDR_BYTE, LSR_ADDR_WORD,
    ROL_REG_NUM_BYTE, ROL_REG_REG_BYTE, ROL_REG_NUM_WORD,
    ROL_REG_REG_WORD, ROL_ADDR_BYTE, ROL_ADDR_WORD,
    ROR_REG_NUM_BYTE, ROR_REG_REG_BYTE, ROR_REG_NUM_WORD,
    ROR_REG_REG_WORD, ROR_ADDR_BYTE, ROR_ADDR_WORD,
    MUL_REG_REG_BYTE, MUL_REG_REG_WORD, MUL_REG_NUM_BYTE, MUL_REG_NUM_WORD,
    MUL_REG_ADDR_BYTE, MUL_REG_ADDR_WORD, MUL_ADDR_REG_BYTE, MUL_ADDR_REG_WORD,
    MUL_ADDR_NUM_BYTE, MUL_ADDR_NUM_WORD, MUL_ADDR_ADDR_BYTE, MUL_ADDR_ADDR_WORD,
    MULS_REG_REG_BYTE, MULS_REG_REG_WORD, MULS_REG_NUM_BYTE, MULS_REG_NUM_WORD,
    MULS_REG_ADDR_BYTE, MULS_REG_ADDR_WORD, MULS_ADDR_REG_BYTE, MULS_ADDR_REG_WORD,
    MULS_ADDR_NUM_BYTE, MULS_ADDR_NUM_WORD, MULS_ADDR_ADDR_BYTE, MULS_ADDR_ADDR_WORD,
    DIV_REG_REG_BYTE, DIV_REG_REG_WORD, DIV_REG_NUM_BYTE, DIV_REG_NUM_WORD,
    DIV_REG_ADDR_BYTE, DIV_REG_ADDR_WORD, DIV_ADDR_REG_BYTE, DIV_ADDR_REG_WORD,
    DIV_ADDR_NUM_BYTE, DIV_ADDR_NUM_WORD, DIV_ADDR_ADDR_BYTE, DIV_ADDR_ADDR_WORD,
    DIVS_REG_REG_BYTE, DIVS_REG_NUM_BYTE, DIVS_REG_ADDR_BYTE, DIVS_ADDR_REG_BYTE,
    DIVS_REG_REG_WORD, DIVS_REG_NUM_WORD, DIVS_REG_ADDR_WORD, DIVS_ADDR_REG_WORD,
    DIVS_ADDR_NUM_BYTE, DIVS_ADDR_NUM_WORD, DIVS_ADDR_ADDR_BYTE, DIVS_ADDR_ADDR_WORD,
    PUSH_REG_BYTE, PUSH_REG_WORD, PUSH_NUM_BYTE, PUSH_NUM_WORD,
    POP_REG_BYTE, POP_REG_WORD,
    JMP_REG, JMP_ADDR,
    JL_REG, JL_ADDR, JLE_REG, JLE_ADDR,
    JG_REG, JG_ADDR, JGE_REG, JGE_ADDR,
    JE_REG, JE_ADDR, JNE_REG, JNE_ADDR,
    CMP_REG_NUM_BYTE, CMP_REG_NUM_WORD, CMP_REG_REG_BYTE, CMP_REG_REG_WORD,
    CMPS_REG_NUM_BYTE, CMPS_REG_NUM_WORD, CMPS_REG_REG_BYTE, CMPS_REG_REG_WORD,
    CMP_REG_ADDR_BYTE, CMP_REG_ADDR_WORD, CMPS_REG_ADDR_BYTE, CMPS_REG_ADDR_WORD,
    JBS_REG_REG, JBS_REG_NUM, JBS_ADDR_REG, JBS_ADDR_NUM,
    JBC_REG_REG, JBC_REG_NUM, JBC_ADDR_REG, JBC_ADDR_NUM,
    JRF_BYTE, JRB_BYTE,
    ADDC_REG_REG_BYTE, ADDC_REG_REG_WORD, ADDC_REG_NUM_BYTE, ADDC_REG_NUM_WORD,
    ADDC_REG_ADDR_BYTE, ADDC_REG_ADDR_WORD, ADDC_ADDR_REG_BYTE, ADDC_ADDR_REG_WORD,
    ADDC_ADDR_NUM_BYTE, ADDC_ADDR_NUM_WORD, ADDC_ADDR_ADDR_BYTE, ADDC_ADDR_ADDR_WORD,
    SUBC_REG_REG_BYTE, SUBC_REG_REG_WORD, SUBC_REG_NUM_BYTE, SUBC_REG_NUM_WORD,
    SUBC_REG_ADDR_BYTE, SUBC_REG_ADDR_WORD, SUBC_ADDR_REG_BYTE, SUBC_ADDR_REG_WORD,
    SUBC_ADDR_NUM_BYTE, SUBC_ADDR_NUM_WORD, SUBC_ADDR_ADDR_BYTE, SUBC_ADDR_ADDR_WORD,
    EHALT,
    SLEEP,
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_all_unique() {
        let mut encountered = vec![];
        for op in ALL {
            if encountered.contains(&op) {
                panic!("Found {op} twice");
            }
            encountered.push(op);
        }
    }
}
