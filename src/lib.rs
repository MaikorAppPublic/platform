use crate::LangError::{InvalidInstructionCode, InvalidInstructionName};
use thiserror::Error;

pub mod constants;
pub mod graphics;
pub mod input;
pub mod mem;
pub mod names;
pub mod op_params;
pub mod ops;
pub mod registers;

#[derive(Error, Debug)]
pub enum LangError {
    #[error("Invalid register name: {0}")]
    InvalidRegisterName(String),
    #[error("Invalid register id: {0}")]
    InvalidRegisterId(u8),
    #[error("Invalid instruction name: {0}")]
    InvalidInstructionName(String),
    #[error("Invalid instruction id: {0}")]
    InvalidInstructionCode(u8),
    #[error("Invalid register argument: {0}")]
    InvalidRegisterArgCode(u8),
}

pub const MAIKOR_VER: [u8; 2] = [0, 1];

pub fn op_codes(text: &str) -> Result<Vec<u8>, LangError> {
    let code = match text.to_ascii_uppercase().as_str() {
        "NOP" => vec![ops::NOP],
        "HALT" => vec![ops::HALT],
        "CALL" => vec![ops::CALL_ADDR, ops::CALL_REG],
        "RET" => vec![ops::RET],
        "RETI" => vec![ops::RETI],
        "CPY.B" => vec![
            ops::CPY_REG_REG_BYTE,
            ops::CPY_ADDR_REG_BYTE,
            ops::CPY_REG_ADDR_BYTE,
            ops::CPY_ADDR_ADDR_BYTE,
            ops::CPY_REG_NUM_BYTE,
            ops::CPY_ADDR_NUM_BYTE,
        ],
        "CPY.W" => vec![
            ops::CPY_REG_REG_WORD,
            ops::CPY_ADDR_REG_WORD,
            ops::CPY_REG_ADDR_WORD,
            ops::CPY_ADDR_ADDR_WORD,
            ops::CPY_REG_NUM_WORD,
            ops::CPY_ADDR_NUM_WORD,
        ],
        "ADD.B" => vec![
            ops::ADD_REG_REG_BYTE,
            ops::ADD_ADDR_REG_BYTE,
            ops::ADD_REG_ADDR_BYTE,
            ops::ADD_ADDR_ADDR_BYTE,
            ops::ADD_REG_NUM_BYTE,
            ops::ADD_ADDR_NUM_BYTE,
        ],
        "ADD.W" => vec![
            ops::ADD_REG_REG_WORD,
            ops::ADD_ADDR_REG_WORD,
            ops::ADD_REG_ADDR_WORD,
            ops::ADD_ADDR_ADDR_WORD,
            ops::ADD_REG_NUM_WORD,
            ops::ADD_ADDR_NUM_WORD,
        ],
        "SUB.B" => vec![
            ops::SUB_REG_REG_BYTE,
            ops::SUB_ADDR_REG_BYTE,
            ops::SUB_REG_ADDR_BYTE,
            ops::SUB_ADDR_ADDR_BYTE,
            ops::SUB_REG_NUM_BYTE,
            ops::SUB_ADDR_NUM_BYTE,
        ],
        "SUB.W" => vec![
            ops::SUB_REG_REG_WORD,
            ops::SUB_ADDR_REG_WORD,
            ops::SUB_REG_ADDR_WORD,
            ops::SUB_ADDR_ADDR_WORD,
            ops::SUB_REG_NUM_WORD,
            ops::SUB_ADDR_NUM_WORD,
        ],
        "MUL.B" => vec![
            ops::MUL_REG_REG_BYTE,
            ops::MUL_ADDR_REG_BYTE,
            ops::MUL_REG_ADDR_BYTE,
            ops::MUL_ADDR_ADDR_BYTE,
            ops::MUL_REG_NUM_BYTE,
            ops::MUL_ADDR_NUM_BYTE,
        ],
        "MUL.W" => vec![
            ops::MUL_REG_REG_WORD,
            ops::MUL_ADDR_REG_WORD,
            ops::MUL_REG_ADDR_WORD,
            ops::MUL_ADDR_ADDR_WORD,
            ops::MUL_REG_NUM_WORD,
            ops::MUL_ADDR_NUM_WORD,
        ],
        "DIV.B" => vec![
            ops::DIV_REG_REG_BYTE,
            ops::DIV_ADDR_REG_BYTE,
            ops::DIV_REG_ADDR_BYTE,
            ops::DIV_ADDR_ADDR_BYTE,
            ops::DIV_REG_NUM_BYTE,
            ops::DIV_ADDR_NUM_BYTE,
        ],
        "DIV.W" => vec![
            ops::DIV_REG_REG_WORD,
            ops::DIV_ADDR_REG_WORD,
            ops::DIV_REG_ADDR_WORD,
            ops::DIV_ADDR_ADDR_WORD,
            ops::DIV_REG_NUM_WORD,
            ops::DIV_ADDR_NUM_WORD,
        ],
        "DIVS.B" => vec![
            ops::DIVS_REG_REG_BYTE,
            ops::DIVS_ADDR_REG_BYTE,
            ops::DIVS_REG_ADDR_BYTE,
            ops::DIVS_ADDR_ADDR_BYTE,
            ops::DIVS_REG_NUM_BYTE,
            ops::DIVS_ADDR_NUM_BYTE,
        ],
        "DIVS.W" => vec![
            ops::DIVS_REG_REG_WORD,
            ops::DIVS_ADDR_REG_WORD,
            ops::DIVS_REG_ADDR_WORD,
            ops::DIVS_ADDR_ADDR_WORD,
            ops::DIVS_REG_NUM_WORD,
            ops::DIVS_ADDR_NUM_WORD,
        ],
        "MULS.B" => vec![
            ops::MULS_REG_REG_BYTE,
            ops::MULS_ADDR_REG_BYTE,
            ops::MULS_REG_ADDR_BYTE,
            ops::MULS_ADDR_ADDR_BYTE,
            ops::MULS_REG_NUM_BYTE,
            ops::MULS_ADDR_NUM_BYTE,
        ],
        "MULS.W" => vec![
            ops::MULS_REG_REG_WORD,
            ops::MULS_ADDR_REG_WORD,
            ops::MULS_REG_ADDR_WORD,
            ops::MULS_ADDR_ADDR_WORD,
            ops::MULS_REG_NUM_WORD,
            ops::MULS_ADDR_NUM_WORD,
        ],
        "ADDC.B" => vec![
            ops::ADDC_REG_REG_BYTE,
            ops::ADDC_ADDR_REG_BYTE,
            ops::ADDC_REG_ADDR_BYTE,
            ops::ADDC_ADDR_ADDR_BYTE,
            ops::ADDC_REG_NUM_BYTE,
            ops::ADDC_ADDR_NUM_BYTE,
        ],
        "ADDC.W" => vec![
            ops::ADDC_REG_REG_WORD,
            ops::ADDC_ADDR_REG_WORD,
            ops::ADDC_REG_ADDR_WORD,
            ops::ADDC_ADDR_ADDR_WORD,
            ops::ADDC_REG_NUM_WORD,
            ops::ADDC_ADDR_NUM_WORD,
        ],
        "SUBC.B" => vec![
            ops::SUBC_REG_REG_BYTE,
            ops::SUBC_ADDR_REG_BYTE,
            ops::SUBC_REG_ADDR_BYTE,
            ops::SUBC_ADDR_ADDR_BYTE,
            ops::SUBC_REG_NUM_BYTE,
            ops::SUBC_ADDR_NUM_BYTE,
        ],
        "SUBC.W" => vec![
            ops::SUBC_REG_REG_WORD,
            ops::SUBC_ADDR_REG_WORD,
            ops::SUBC_REG_ADDR_WORD,
            ops::SUBC_ADDR_ADDR_WORD,
            ops::SUBC_REG_NUM_WORD,
            ops::SUBC_ADDR_NUM_WORD,
        ],
        "INC.B" => vec![ops::INC_REG_BYTE, ops::INC_ADDR_BYTE],
        "INC.W" => vec![ops::INC_REG_WORD, ops::INC_ADDR_WORD],
        "DEC.B" => vec![ops::DEC_REG_BYTE, ops::DEC_ADDR_BYTE],
        "DEC.W" => vec![ops::DEC_REG_WORD, ops::DEC_ADDR_WORD],
        "SWAP.B" => vec![ops::SWAP_REG_REG_BYTE],
        "SWAP.W" => vec![ops::SWAP_REG_REG_WORD],
        "MCPY" => vec![
            ops::MEM_CPY_REG_ADDR_BYTE,
            ops::MEM_CPY_REG_REG_BYTE,
            ops::MEM_CPY_ADDR_REG_BYTE,
            ops::MEM_CPY_ADDR_REG_REG,
            ops::MEM_CPY_REG_ADDR_REG,
            ops::MEM_CPY_ADDR_ADDR_REG,
            ops::MEM_CPY_ADDR_ADDR_BYTE,
            ops::MEM_CPY_REG_REG_REG,
        ],
        "NOT.B" => vec![ops::NOT_REG_BYTE],
        "NOT.W" => vec![ops::NOT_REG_WORD],
        "AND.B" => vec![ops::AND_REG_REG_BYTE, ops::AND_REG_NUM_BYTE],
        "AND.W" => vec![ops::AND_REG_REG_WORD, ops::AND_REG_NUM_WORD],
        "OR.B" => vec![ops::OR_REG_REG_BYTE, ops::OR_REG_NUM_BYTE],
        "OR.W" => vec![ops::OR_REG_REG_WORD, ops::OR_REG_NUM_WORD],
        "XOR.B" => vec![ops::XOR_REG_REG_BYTE, ops::XOR_REG_NUM_BYTE],
        "XOR.W" => vec![ops::XOR_REG_REG_WORD, ops::XOR_REG_NUM_WORD],
        "ASL.B" => vec![
            ops::ASL_REG_NUM_BYTE,
            ops::ASL_REG_REG_BYTE,
            ops::ASL_ADDR_BYTE,
        ],
        "ASL.W" => vec![
            ops::ASL_REG_NUM_WORD,
            ops::ASL_REG_REG_WORD,
            ops::ASL_ADDR_WORD,
        ],
        "ASR.B" => vec![
            ops::ASR_REG_NUM_BYTE,
            ops::ASR_REG_REG_BYTE,
            ops::ASR_ADDR_BYTE,
        ],
        "ASR.W" => vec![
            ops::ASR_REG_NUM_WORD,
            ops::ASR_REG_REG_WORD,
            ops::ASR_ADDR_WORD,
        ],
        "LSR.B" => vec![
            ops::LSR_REG_NUM_BYTE,
            ops::LSR_REG_REG_BYTE,
            ops::LSR_ADDR_BYTE,
        ],
        "LSR.W" => vec![
            ops::LSR_REG_NUM_WORD,
            ops::LSR_REG_REG_WORD,
            ops::LSR_ADDR_WORD,
        ],
        "ROL.B" => vec![
            ops::ROL_REG_NUM_BYTE,
            ops::ROL_REG_REG_BYTE,
            ops::ROL_ADDR_BYTE,
        ],
        "ROL.W" => vec![
            ops::ROL_REG_NUM_WORD,
            ops::ROL_REG_REG_WORD,
            ops::ROL_ADDR_WORD,
        ],
        "ROR.B" => vec![
            ops::ROR_REG_NUM_BYTE,
            ops::ROR_REG_REG_BYTE,
            ops::ROR_ADDR_BYTE,
        ],
        "ROR.W" => vec![
            ops::ROR_REG_NUM_WORD,
            ops::ROR_REG_REG_WORD,
            ops::ROR_ADDR_WORD,
        ],
        "JMP" => vec![ops::JMP_REG, ops::JMP_ADDR],
        "JE" => vec![ops::JE_REG, ops::JE_ADDR],
        "JNE" => vec![ops::JNE_REG, ops::JNE_ADDR],
        "JL" => vec![ops::JL_REG, ops::JL_ADDR],
        "JG" => vec![ops::JG_REG, ops::JG_ADDR],
        "JLE" => vec![ops::JLE_REG, ops::JLE_ADDR],
        "JGE" => vec![ops::JGE_REG, ops::JGE_ADDR],
        "JRF" => vec![ops::JRF_BYTE],
        "JRB" => vec![ops::JRB_BYTE],
        "JBS" => vec![
            ops::JBS_REG_NUM,
            ops::JBS_REG_REG,
            ops::JBS_ADDR_REG,
            ops::JBS_ADDR_NUM,
        ],
        "JBC" => vec![
            ops::JBC_REG_NUM,
            ops::JBC_REG_REG,
            ops::JBC_ADDR_REG,
            ops::JBC_ADDR_NUM,
        ],
        "CMP.B" => vec![
            ops::CMP_REG_NUM_BYTE,
            ops::CMP_REG_REG_BYTE,
            ops::CMP_REG_ADDR_BYTE,
        ],
        "CMP.W" => vec![
            ops::CMP_REG_NUM_WORD,
            ops::CMP_REG_REG_WORD,
            ops::CMP_REG_ADDR_WORD,
        ],
        "CMPS.B" => vec![
            ops::CMPS_REG_NUM_BYTE,
            ops::CMPS_REG_REG_BYTE,
            ops::CMPS_REG_ADDR_BYTE,
        ],
        "CMPS.W" => vec![
            ops::CMPS_REG_NUM_WORD,
            ops::CMPS_REG_REG_WORD,
            ops::CMPS_REG_ADDR_WORD,
        ],
        "PUSH.B" => vec![ops::PUSH_REG_BYTE, ops::PUSH_NUM_BYTE],
        "PUSH.W" => vec![ops::PUSH_REG_WORD, ops::PUSH_NUM_WORD],
        "POP.B" => vec![ops::POP_REG_BYTE],
        "POP.W" => vec![ops::POP_REG_WORD],
        _ => return Err(InvalidInstructionName(text.to_string())),
    };
    Ok(code)
}

pub fn op_name(op: u8) -> Result<&'static str, LangError> {
    let name = match op {
        ops::NOP => names::op::NOP,
        ops::HALT => names::op::HALT,
        ops::EHALT => names::op::EHALT,
        ops::CALL_ADDR => names::op::CALL_ADDR,
        ops::CALL_REG => names::op::CALL_REG,
        ops::RET => names::op::RET,
        ops::RETI => names::op::RETI,
        ops::CPY_REG_REG_BYTE => names::op::CPY_REG_REG_BYTE,
        ops::CPY_REG_REG_WORD => names::op::CPY_REG_REG_WORD,
        ops::CPY_ADDR_REG_BYTE => names::op::CPY_ADDR_REG_BYTE,
        ops::CPY_ADDR_REG_WORD => names::op::CPY_ADDR_REG_WORD,
        ops::CPY_REG_ADDR_BYTE => names::op::CPY_REG_ADDR_BYTE,
        ops::CPY_REG_ADDR_WORD => names::op::CPY_REG_ADDR_WORD,
        ops::CPY_ADDR_ADDR_BYTE => names::op::CPY_ADDR_ADDR_BYTE,
        ops::CPY_ADDR_ADDR_WORD => names::op::CPY_ADDR_ADDR_WORD,
        ops::CPY_REG_NUM_BYTE => names::op::CPY_REG_NUM_BYTE,
        ops::CPY_REG_NUM_WORD => names::op::CPY_REG_NUM_WORD,
        ops::CPY_ADDR_NUM_BYTE => names::op::CPY_ADDR_NUM_BYTE,
        ops::CPY_ADDR_NUM_WORD => names::op::CPY_ADDR_NUM_WORD,
        ops::ADD_REG_REG_BYTE => names::op::ADD_REG_REG_BYTE,
        ops::ADD_REG_REG_WORD => names::op::ADD_REG_REG_WORD,
        ops::ADD_REG_NUM_BYTE => names::op::ADD_REG_NUM_BYTE,
        ops::ADD_REG_NUM_WORD => names::op::ADD_REG_NUM_WORD,
        ops::ADD_REG_ADDR_BYTE => names::op::ADD_REG_ADDR_BYTE,
        ops::ADD_REG_ADDR_WORD => names::op::ADD_REG_ADDR_WORD,
        ops::ADD_ADDR_REG_BYTE => names::op::ADD_ADDR_REG_BYTE,
        ops::ADD_ADDR_REG_WORD => names::op::ADD_ADDR_REG_WORD,
        ops::ADD_ADDR_NUM_BYTE => names::op::ADD_ADDR_NUM_BYTE,
        ops::ADD_ADDR_NUM_WORD => names::op::ADD_ADDR_NUM_WORD,
        ops::ADD_ADDR_ADDR_BYTE => names::op::ADD_ADDR_ADDR_BYTE,
        ops::ADD_ADDR_ADDR_WORD => names::op::ADD_ADDR_ADDR_WORD,
        ops::INC_REG_BYTE => names::op::INC_REG_BYTE,
        ops::INC_REG_WORD => names::op::INC_REG_WORD,
        ops::INC_ADDR_BYTE => names::op::INC_ADDR_BYTE,
        ops::INC_ADDR_WORD => names::op::INC_ADDR_WORD,
        ops::SUB_REG_REG_BYTE => names::op::SUB_REG_REG_BYTE,
        ops::SUB_REG_REG_WORD => names::op::SUB_REG_REG_WORD,
        ops::SUB_REG_NUM_BYTE => names::op::SUB_REG_NUM_BYTE,
        ops::SUB_REG_NUM_WORD => names::op::SUB_REG_NUM_WORD,
        ops::SUB_REG_ADDR_BYTE => names::op::SUB_REG_ADDR_BYTE,
        ops::SUB_REG_ADDR_WORD => names::op::SUB_REG_ADDR_WORD,
        ops::SUB_ADDR_REG_BYTE => names::op::SUB_ADDR_REG_BYTE,
        ops::SUB_ADDR_REG_WORD => names::op::SUB_ADDR_REG_WORD,
        ops::SUB_ADDR_NUM_BYTE => names::op::SUB_ADDR_NUM_BYTE,
        ops::SUB_ADDR_NUM_WORD => names::op::SUB_ADDR_NUM_WORD,
        ops::SUB_ADDR_ADDR_BYTE => names::op::SUB_ADDR_ADDR_BYTE,
        ops::SUB_ADDR_ADDR_WORD => names::op::SUB_ADDR_ADDR_WORD,
        ops::DEC_REG_BYTE => names::op::DEC_REG_BYTE,
        ops::DEC_REG_WORD => names::op::DEC_REG_WORD,
        ops::DEC_ADDR_BYTE => names::op::DEC_ADDR_BYTE,
        ops::DEC_ADDR_WORD => names::op::DEC_ADDR_WORD,
        ops::SWAP_REG_REG_BYTE => names::op::SWAP_REG_REG_BYTE,
        ops::SWAP_REG_REG_WORD => names::op::SWAP_REG_REG_WORD,
        ops::MEM_CPY_ADDR_ADDR_BYTE => names::op::MEM_CPY_ADDR_ADDR_BYTE,
        ops::MEM_CPY_ADDR_REG_BYTE => names::op::MEM_CPY_ADDR_REG_BYTE,
        ops::MEM_CPY_REG_ADDR_BYTE => names::op::MEM_CPY_REG_ADDR_BYTE,
        ops::MEM_CPY_REG_REG_BYTE => names::op::MEM_CPY_REG_REG_BYTE,
        ops::MEM_CPY_ADDR_ADDR_REG => names::op::MEM_CPY_ADDR_ADDR_REG,
        ops::MEM_CPY_ADDR_REG_REG => names::op::MEM_CPY_ADDR_REG_REG,
        ops::MEM_CPY_REG_ADDR_REG => names::op::MEM_CPY_REG_ADDR_REG,
        ops::MEM_CPY_REG_REG_REG => names::op::MEM_CPY_REG_REG_REG,
        ops::NOT_REG_BYTE => names::op::NOT_REG_BYTE,
        ops::NOT_REG_WORD => names::op::NOT_REG_WORD,
        ops::OR_REG_REG_BYTE => names::op::OR_REG_REG_BYTE,
        ops::OR_REG_REG_WORD => names::op::OR_REG_REG_WORD,
        ops::OR_REG_NUM_BYTE => names::op::OR_REG_NUM_BYTE,
        ops::OR_REG_NUM_WORD => names::op::OR_REG_NUM_WORD,
        ops::XOR_REG_REG_BYTE => names::op::XOR_REG_REG_BYTE,
        ops::XOR_REG_REG_WORD => names::op::XOR_REG_REG_WORD,
        ops::XOR_REG_NUM_BYTE => names::op::XOR_REG_NUM_BYTE,
        ops::XOR_REG_NUM_WORD => names::op::XOR_REG_NUM_WORD,
        ops::AND_REG_REG_BYTE => names::op::AND_REG_REG_BYTE,
        ops::AND_REG_REG_WORD => names::op::AND_REG_REG_WORD,
        ops::AND_REG_NUM_BYTE => names::op::AND_REG_NUM_BYTE,
        ops::AND_REG_NUM_WORD => names::op::AND_REG_NUM_WORD,
        ops::ASL_REG_NUM_BYTE => names::op::ASL_REG_NUM_BYTE,
        ops::ASL_REG_REG_BYTE => names::op::ASL_REG_REG_BYTE,
        ops::ASL_REG_NUM_WORD => names::op::ASL_REG_NUM_WORD,
        ops::ASL_REG_REG_WORD => names::op::ASL_REG_REG_WORD,
        ops::ASL_ADDR_BYTE => names::op::ASL_ADDR_BYTE,
        ops::ASL_ADDR_WORD => names::op::ASL_ADDR_WORD,
        ops::ASR_REG_NUM_BYTE => names::op::ASR_REG_NUM_BYTE,
        ops::ASR_REG_REG_BYTE => names::op::ASR_REG_REG_BYTE,
        ops::ASR_REG_NUM_WORD => names::op::ASR_REG_NUM_WORD,
        ops::ASR_REG_REG_WORD => names::op::ASR_REG_REG_WORD,
        ops::ASR_ADDR_BYTE => names::op::ASR_ADDR_BYTE,
        ops::ASR_ADDR_WORD => names::op::ASR_ADDR_WORD,
        ops::LSR_REG_NUM_BYTE => names::op::LSR_REG_NUM_BYTE,
        ops::LSR_REG_REG_BYTE => names::op::LSR_REG_REG_BYTE,
        ops::LSR_REG_NUM_WORD => names::op::LSR_REG_NUM_WORD,
        ops::LSR_REG_REG_WORD => names::op::LSR_REG_REG_WORD,
        ops::LSR_ADDR_BYTE => names::op::LSR_ADDR_BYTE,
        ops::LSR_ADDR_WORD => names::op::LSR_ADDR_WORD,
        ops::ROL_REG_NUM_BYTE => names::op::ROL_REG_NUM_BYTE,
        ops::ROL_REG_REG_BYTE => names::op::ROL_REG_REG_BYTE,
        ops::ROL_REG_NUM_WORD => names::op::ROL_REG_NUM_WORD,
        ops::ROL_REG_REG_WORD => names::op::ROL_REG_REG_WORD,
        ops::ROL_ADDR_BYTE => names::op::ROL_ADDR_BYTE,
        ops::ROL_ADDR_WORD => names::op::ROL_ADDR_WORD,
        ops::ROR_REG_NUM_BYTE => names::op::ROR_REG_NUM_BYTE,
        ops::ROR_REG_REG_BYTE => names::op::ROR_REG_REG_BYTE,
        ops::ROR_REG_NUM_WORD => names::op::ROR_REG_NUM_WORD,
        ops::ROR_REG_REG_WORD => names::op::ROR_REG_REG_WORD,
        ops::ROR_ADDR_BYTE => names::op::ROR_ADDR_BYTE,
        ops::ROR_ADDR_WORD => names::op::ROR_ADDR_WORD,
        ops::MUL_REG_REG_BYTE => names::op::MUL_REG_REG_BYTE,
        ops::MUL_REG_REG_WORD => names::op::MUL_REG_REG_WORD,
        ops::MUL_REG_NUM_BYTE => names::op::MUL_REG_NUM_BYTE,
        ops::MUL_REG_NUM_WORD => names::op::MUL_REG_NUM_WORD,
        ops::MUL_REG_ADDR_BYTE => names::op::MUL_REG_ADDR_BYTE,
        ops::MUL_REG_ADDR_WORD => names::op::MUL_REG_ADDR_WORD,
        ops::MUL_ADDR_REG_BYTE => names::op::MUL_ADDR_REG_BYTE,
        ops::MUL_ADDR_REG_WORD => names::op::MUL_ADDR_REG_WORD,
        ops::MUL_ADDR_NUM_BYTE => names::op::MUL_ADDR_NUM_BYTE,
        ops::MUL_ADDR_NUM_WORD => names::op::MUL_ADDR_NUM_WORD,
        ops::MUL_ADDR_ADDR_BYTE => names::op::MUL_ADDR_ADDR_BYTE,
        ops::MUL_ADDR_ADDR_WORD => names::op::MUL_ADDR_ADDR_WORD,
        ops::MULS_REG_REG_BYTE => names::op::MULS_REG_REG_BYTE,
        ops::MULS_REG_REG_WORD => names::op::MULS_REG_REG_WORD,
        ops::MULS_REG_NUM_BYTE => names::op::MULS_REG_NUM_BYTE,
        ops::MULS_REG_NUM_WORD => names::op::MULS_REG_NUM_WORD,
        ops::MULS_REG_ADDR_BYTE => names::op::MULS_REG_ADDR_BYTE,
        ops::MULS_REG_ADDR_WORD => names::op::MULS_REG_ADDR_WORD,
        ops::MULS_ADDR_REG_BYTE => names::op::MULS_ADDR_REG_BYTE,
        ops::MULS_ADDR_REG_WORD => names::op::MULS_ADDR_REG_WORD,
        ops::MULS_ADDR_NUM_BYTE => names::op::MULS_ADDR_NUM_BYTE,
        ops::MULS_ADDR_NUM_WORD => names::op::MULS_ADDR_NUM_WORD,
        ops::MULS_ADDR_ADDR_BYTE => names::op::MULS_ADDR_ADDR_BYTE,
        ops::MULS_ADDR_ADDR_WORD => names::op::MULS_ADDR_ADDR_WORD,
        ops::DIV_REG_REG_BYTE => names::op::DIV_REG_REG_BYTE,
        ops::DIV_REG_REG_WORD => names::op::DIV_REG_REG_WORD,
        ops::DIV_REG_NUM_BYTE => names::op::DIV_REG_NUM_BYTE,
        ops::DIV_REG_NUM_WORD => names::op::DIV_REG_NUM_WORD,
        ops::DIV_REG_ADDR_BYTE => names::op::DIV_REG_ADDR_BYTE,
        ops::DIV_REG_ADDR_WORD => names::op::DIV_REG_ADDR_WORD,
        ops::DIV_ADDR_REG_BYTE => names::op::DIV_ADDR_REG_BYTE,
        ops::DIV_ADDR_REG_WORD => names::op::DIV_ADDR_REG_WORD,
        ops::DIV_ADDR_NUM_BYTE => names::op::DIV_ADDR_NUM_BYTE,
        ops::DIV_ADDR_NUM_WORD => names::op::DIV_ADDR_NUM_WORD,
        ops::DIV_ADDR_ADDR_BYTE => names::op::DIV_ADDR_ADDR_BYTE,
        ops::DIV_ADDR_ADDR_WORD => names::op::DIV_ADDR_ADDR_WORD,
        ops::DIVS_REG_REG_BYTE => names::op::DIVS_REG_REG_BYTE,
        ops::DIVS_REG_NUM_BYTE => names::op::DIVS_REG_NUM_BYTE,
        ops::DIVS_REG_ADDR_BYTE => names::op::DIVS_REG_ADDR_BYTE,
        ops::DIVS_ADDR_REG_BYTE => names::op::DIVS_ADDR_REG_BYTE,
        ops::DIVS_REG_REG_WORD => names::op::DIVS_REG_REG_WORD,
        ops::DIVS_REG_NUM_WORD => names::op::DIVS_REG_NUM_WORD,
        ops::DIVS_REG_ADDR_WORD => names::op::DIVS_REG_ADDR_WORD,
        ops::DIVS_ADDR_REG_WORD => names::op::DIVS_ADDR_REG_WORD,
        ops::DIVS_ADDR_NUM_BYTE => names::op::DIVS_ADDR_NUM_BYTE,
        ops::DIVS_ADDR_NUM_WORD => names::op::DIVS_ADDR_NUM_WORD,
        ops::DIVS_ADDR_ADDR_BYTE => names::op::DIVS_ADDR_ADDR_BYTE,
        ops::DIVS_ADDR_ADDR_WORD => names::op::DIVS_ADDR_ADDR_WORD,
        ops::PUSH_REG_BYTE => names::op::PUSH_REG_BYTE,
        ops::PUSH_REG_WORD => names::op::PUSH_REG_WORD,
        ops::PUSH_NUM_BYTE => names::op::PUSH_NUM_BYTE,
        ops::PUSH_NUM_WORD => names::op::PUSH_NUM_WORD,
        ops::POP_REG_BYTE => names::op::POP_REG_BYTE,
        ops::POP_REG_WORD => names::op::POP_REG_WORD,
        ops::JMP_REG => names::op::JMP_REG,
        ops::JMP_ADDR => names::op::JMP_ADDR,
        ops::JL_REG => names::op::JL_REG,
        ops::JL_ADDR => names::op::JL_ADDR,
        ops::JLE_REG => names::op::JLE_REG,
        ops::JLE_ADDR => names::op::JLE_ADDR,
        ops::JG_REG => names::op::JG_REG,
        ops::JG_ADDR => names::op::JG_ADDR,
        ops::JGE_REG => names::op::JGE_REG,
        ops::JGE_ADDR => names::op::JGE_ADDR,
        ops::JE_REG => names::op::JE_REG,
        ops::JE_ADDR => names::op::JE_ADDR,
        ops::JNE_REG => names::op::JNE_REG,
        ops::JNE_ADDR => names::op::JNE_ADDR,
        ops::CMP_REG_NUM_BYTE => names::op::CMP_REG_NUM_BYTE,
        ops::CMP_REG_NUM_WORD => names::op::CMP_REG_NUM_WORD,
        ops::CMP_REG_REG_BYTE => names::op::CMP_REG_REG_BYTE,
        ops::CMP_REG_REG_WORD => names::op::CMP_REG_REG_WORD,
        ops::CMPS_REG_NUM_BYTE => names::op::CMPS_REG_NUM_BYTE,
        ops::CMPS_REG_NUM_WORD => names::op::CMPS_REG_NUM_WORD,
        ops::CMPS_REG_REG_BYTE => names::op::CMPS_REG_REG_BYTE,
        ops::CMPS_REG_REG_WORD => names::op::CMPS_REG_REG_WORD,
        ops::JBS_REG_REG => names::op::JBS_REG_REG,
        ops::JBS_REG_NUM => names::op::JBS_REG_NUM,
        ops::JBS_ADDR_REG => names::op::JBS_ADDR_REG,
        ops::JBS_ADDR_NUM => names::op::JBS_ADDR_NUM,
        ops::JBC_REG_REG => names::op::JBC_REG_REG,
        ops::JBC_REG_NUM => names::op::JBC_REG_NUM,
        ops::JBC_ADDR_REG => names::op::JBC_ADDR_REG,
        ops::JBC_ADDR_NUM => names::op::JBC_ADDR_NUM,
        ops::CMP_REG_ADDR_BYTE => names::op::CMP_REG_ADDR_BYTE,
        ops::CMP_REG_ADDR_WORD => names::op::CMP_REG_ADDR_WORD,
        ops::CMPS_REG_ADDR_BYTE => names::op::CMPS_REG_ADDR_BYTE,
        ops::CMPS_REG_ADDR_WORD => names::op::CMPS_REG_ADDR_WORD,
        ops::JRF_BYTE => names::op::JRF_BYTE,
        ops::JRB_BYTE => names::op::JRB_BYTE,
        ops::ADDC_REG_REG_BYTE => names::op::ADDC_REG_REG_BYTE,
        ops::ADDC_REG_REG_WORD => names::op::ADDC_REG_REG_WORD,
        ops::ADDC_REG_NUM_BYTE => names::op::ADDC_REG_NUM_BYTE,
        ops::ADDC_REG_NUM_WORD => names::op::ADDC_REG_NUM_WORD,
        ops::ADDC_REG_ADDR_BYTE => names::op::ADDC_REG_ADDR_BYTE,
        ops::ADDC_REG_ADDR_WORD => names::op::ADDC_REG_ADDR_WORD,
        ops::ADDC_ADDR_REG_BYTE => names::op::ADDC_ADDR_REG_BYTE,
        ops::ADDC_ADDR_REG_WORD => names::op::ADDC_ADDR_REG_WORD,
        ops::ADDC_ADDR_NUM_BYTE => names::op::ADDC_ADDR_NUM_BYTE,
        ops::ADDC_ADDR_NUM_WORD => names::op::ADDC_ADDR_NUM_WORD,
        ops::ADDC_ADDR_ADDR_BYTE => names::op::ADDC_ADDR_ADDR_BYTE,
        ops::ADDC_ADDR_ADDR_WORD => names::op::ADDC_ADDR_ADDR_WORD,
        ops::SUBC_REG_REG_BYTE => names::op::SUBC_REG_REG_BYTE,
        ops::SUBC_REG_REG_WORD => names::op::SUBC_REG_REG_WORD,
        ops::SUBC_REG_NUM_BYTE => names::op::SUBC_REG_NUM_BYTE,
        ops::SUBC_REG_NUM_WORD => names::op::SUBC_REG_NUM_WORD,
        ops::SUBC_REG_ADDR_BYTE => names::op::SUBC_REG_ADDR_BYTE,
        ops::SUBC_REG_ADDR_WORD => names::op::SUBC_REG_ADDR_WORD,
        ops::SUBC_ADDR_REG_BYTE => names::op::SUBC_ADDR_REG_BYTE,
        ops::SUBC_ADDR_REG_WORD => names::op::SUBC_ADDR_REG_WORD,
        ops::SUBC_ADDR_NUM_BYTE => names::op::SUBC_ADDR_NUM_BYTE,
        ops::SUBC_ADDR_NUM_WORD => names::op::SUBC_ADDR_NUM_WORD,
        ops::SUBC_ADDR_ADDR_BYTE => names::op::SUBC_ADDR_ADDR_BYTE,
        ops::SUBC_ADDR_ADDR_WORD => names::op::SUBC_ADDR_ADDR_WORD,
        ops::SLEEP => names::op::SLEEP,
        _ => return Err(InvalidInstructionCode(op)),
    };
    Ok(name)
}

pub fn op_desc(op: u8) -> Result<&'static str, LangError> {
    let name = match op {
        ops::NOP => names::full::NOP,
        ops::HALT => names::full::HALT,
        ops::EHALT => names::full::EHALT,
        ops::CALL_ADDR => names::full::CALL_ADDR,
        ops::CALL_REG => names::full::CALL_REG,
        ops::RET => names::full::RET,
        ops::RETI => names::full::RETI,
        ops::CPY_REG_REG_BYTE => names::full::CPY_REG_REG_BYTE,
        ops::CPY_REG_REG_WORD => names::full::CPY_REG_REG_WORD,
        ops::CPY_ADDR_REG_BYTE => names::full::CPY_ADDR_REG_BYTE,
        ops::CPY_ADDR_REG_WORD => names::full::CPY_ADDR_REG_WORD,
        ops::CPY_REG_ADDR_BYTE => names::full::CPY_REG_ADDR_BYTE,
        ops::CPY_REG_ADDR_WORD => names::full::CPY_REG_ADDR_WORD,
        ops::CPY_ADDR_ADDR_BYTE => names::full::CPY_ADDR_ADDR_BYTE,
        ops::CPY_ADDR_ADDR_WORD => names::full::CPY_ADDR_ADDR_WORD,
        ops::CPY_REG_NUM_BYTE => names::full::CPY_REG_NUM_BYTE,
        ops::CPY_REG_NUM_WORD => names::full::CPY_REG_NUM_WORD,
        ops::CPY_ADDR_NUM_BYTE => names::full::CPY_ADDR_NUM_BYTE,
        ops::CPY_ADDR_NUM_WORD => names::full::CPY_ADDR_NUM_WORD,
        ops::ADD_REG_REG_BYTE => names::full::ADD_REG_REG_BYTE,
        ops::ADD_REG_REG_WORD => names::full::ADD_REG_REG_WORD,
        ops::ADD_REG_NUM_BYTE => names::full::ADD_REG_NUM_BYTE,
        ops::ADD_REG_NUM_WORD => names::full::ADD_REG_NUM_WORD,
        ops::ADD_REG_ADDR_BYTE => names::full::ADD_REG_ADDR_BYTE,
        ops::ADD_REG_ADDR_WORD => names::full::ADD_REG_ADDR_WORD,
        ops::ADD_ADDR_REG_BYTE => names::full::ADD_ADDR_REG_BYTE,
        ops::ADD_ADDR_REG_WORD => names::full::ADD_ADDR_REG_WORD,
        ops::ADD_ADDR_NUM_BYTE => names::full::ADD_ADDR_NUM_BYTE,
        ops::ADD_ADDR_NUM_WORD => names::full::ADD_ADDR_NUM_WORD,
        ops::ADD_ADDR_ADDR_BYTE => names::full::ADD_ADDR_ADDR_BYTE,
        ops::ADD_ADDR_ADDR_WORD => names::full::ADD_ADDR_ADDR_WORD,
        ops::INC_REG_BYTE => names::full::INC_REG_BYTE,
        ops::INC_REG_WORD => names::full::INC_REG_WORD,
        ops::INC_ADDR_BYTE => names::full::INC_ADDR_BYTE,
        ops::INC_ADDR_WORD => names::full::INC_ADDR_WORD,
        ops::SUB_REG_REG_BYTE => names::full::SUB_REG_REG_BYTE,
        ops::SUB_REG_REG_WORD => names::full::SUB_REG_REG_WORD,
        ops::SUB_REG_NUM_BYTE => names::full::SUB_REG_NUM_BYTE,
        ops::SUB_REG_NUM_WORD => names::full::SUB_REG_NUM_WORD,
        ops::SUB_REG_ADDR_BYTE => names::full::SUB_REG_ADDR_BYTE,
        ops::SUB_REG_ADDR_WORD => names::full::SUB_REG_ADDR_WORD,
        ops::SUB_ADDR_REG_BYTE => names::full::SUB_ADDR_REG_BYTE,
        ops::SUB_ADDR_REG_WORD => names::full::SUB_ADDR_REG_WORD,
        ops::SUB_ADDR_NUM_BYTE => names::full::SUB_ADDR_NUM_BYTE,
        ops::SUB_ADDR_NUM_WORD => names::full::SUB_ADDR_NUM_WORD,
        ops::SUB_ADDR_ADDR_BYTE => names::full::SUB_ADDR_ADDR_BYTE,
        ops::SUB_ADDR_ADDR_WORD => names::full::SUB_ADDR_ADDR_WORD,
        ops::DEC_REG_BYTE => names::full::DEC_REG_BYTE,
        ops::DEC_REG_WORD => names::full::DEC_REG_WORD,
        ops::DEC_ADDR_BYTE => names::full::DEC_ADDR_BYTE,
        ops::DEC_ADDR_WORD => names::full::DEC_ADDR_WORD,
        ops::SWAP_REG_REG_BYTE => names::full::SWAP_REG_REG_BYTE,
        ops::SWAP_REG_REG_WORD => names::full::SWAP_REG_REG_WORD,
        ops::MEM_CPY_ADDR_ADDR_BYTE => names::full::MEM_CPY_ADDR_ADDR_BYTE,
        ops::MEM_CPY_ADDR_REG_BYTE => names::full::MEM_CPY_ADDR_REG_BYTE,
        ops::MEM_CPY_REG_ADDR_BYTE => names::full::MEM_CPY_REG_ADDR_BYTE,
        ops::MEM_CPY_REG_REG_BYTE => names::full::MEM_CPY_REG_REG_BYTE,
        ops::MEM_CPY_ADDR_ADDR_REG => names::full::MEM_CPY_ADDR_ADDR_REG,
        ops::MEM_CPY_ADDR_REG_REG => names::full::MEM_CPY_ADDR_REG_REG,
        ops::MEM_CPY_REG_ADDR_REG => names::full::MEM_CPY_REG_ADDR_REG,
        ops::MEM_CPY_REG_REG_REG => names::full::MEM_CPY_REG_REG_REG,
        ops::NOT_REG_BYTE => names::full::NOT_REG_BYTE,
        ops::NOT_REG_WORD => names::full::NOT_REG_WORD,
        ops::OR_REG_REG_BYTE => names::full::OR_REG_REG_BYTE,
        ops::OR_REG_REG_WORD => names::full::OR_REG_REG_WORD,
        ops::OR_REG_NUM_BYTE => names::full::OR_REG_NUM_BYTE,
        ops::OR_REG_NUM_WORD => names::full::OR_REG_NUM_WORD,
        ops::XOR_REG_REG_BYTE => names::full::XOR_REG_REG_BYTE,
        ops::XOR_REG_REG_WORD => names::full::XOR_REG_REG_WORD,
        ops::XOR_REG_NUM_BYTE => names::full::XOR_REG_NUM_BYTE,
        ops::XOR_REG_NUM_WORD => names::full::XOR_REG_NUM_WORD,
        ops::AND_REG_REG_BYTE => names::full::AND_REG_REG_BYTE,
        ops::AND_REG_REG_WORD => names::full::AND_REG_REG_WORD,
        ops::AND_REG_NUM_BYTE => names::full::AND_REG_NUM_BYTE,
        ops::AND_REG_NUM_WORD => names::full::AND_REG_NUM_WORD,
        ops::ASL_REG_NUM_BYTE => names::full::ASL_REG_NUM_BYTE,
        ops::ASL_REG_REG_BYTE => names::full::ASL_REG_REG_BYTE,
        ops::ASL_REG_NUM_WORD => names::full::ASL_REG_NUM_WORD,
        ops::ASL_REG_REG_WORD => names::full::ASL_REG_REG_WORD,
        ops::ASL_ADDR_BYTE => names::full::ASL_ADDR_BYTE,
        ops::ASL_ADDR_WORD => names::full::ASL_ADDR_WORD,
        ops::ASR_REG_NUM_BYTE => names::full::ASR_REG_NUM_BYTE,
        ops::ASR_REG_REG_BYTE => names::full::ASR_REG_REG_BYTE,
        ops::ASR_REG_NUM_WORD => names::full::ASR_REG_NUM_WORD,
        ops::ASR_REG_REG_WORD => names::full::ASR_REG_REG_WORD,
        ops::ASR_ADDR_BYTE => names::full::ASR_ADDR_BYTE,
        ops::ASR_ADDR_WORD => names::full::ASR_ADDR_WORD,
        ops::LSR_REG_NUM_BYTE => names::full::LSR_REG_NUM_BYTE,
        ops::LSR_REG_REG_BYTE => names::full::LSR_REG_REG_BYTE,
        ops::LSR_REG_NUM_WORD => names::full::LSR_REG_NUM_WORD,
        ops::LSR_REG_REG_WORD => names::full::LSR_REG_REG_WORD,
        ops::LSR_ADDR_BYTE => names::full::LSR_ADDR_BYTE,
        ops::LSR_ADDR_WORD => names::full::LSR_ADDR_WORD,
        ops::ROL_REG_NUM_BYTE => names::full::ROL_REG_NUM_BYTE,
        ops::ROL_REG_REG_BYTE => names::full::ROL_REG_REG_BYTE,
        ops::ROL_REG_NUM_WORD => names::full::ROL_REG_NUM_WORD,
        ops::ROL_REG_REG_WORD => names::full::ROL_REG_REG_WORD,
        ops::ROL_ADDR_BYTE => names::full::ROL_ADDR_BYTE,
        ops::ROL_ADDR_WORD => names::full::ROL_ADDR_WORD,
        ops::ROR_REG_NUM_BYTE => names::full::ROR_REG_NUM_BYTE,
        ops::ROR_REG_REG_BYTE => names::full::ROR_REG_REG_BYTE,
        ops::ROR_REG_NUM_WORD => names::full::ROR_REG_NUM_WORD,
        ops::ROR_REG_REG_WORD => names::full::ROR_REG_REG_WORD,
        ops::ROR_ADDR_BYTE => names::full::ROR_ADDR_BYTE,
        ops::ROR_ADDR_WORD => names::full::ROR_ADDR_WORD,
        ops::MUL_REG_REG_BYTE => names::full::MUL_REG_REG_BYTE,
        ops::MUL_REG_REG_WORD => names::full::MUL_REG_REG_WORD,
        ops::MUL_REG_NUM_BYTE => names::full::MUL_REG_NUM_BYTE,
        ops::MUL_REG_NUM_WORD => names::full::MUL_REG_NUM_WORD,
        ops::MUL_REG_ADDR_BYTE => names::full::MUL_REG_ADDR_BYTE,
        ops::MUL_REG_ADDR_WORD => names::full::MUL_REG_ADDR_WORD,
        ops::MUL_ADDR_REG_BYTE => names::full::MUL_ADDR_REG_BYTE,
        ops::MUL_ADDR_REG_WORD => names::full::MUL_ADDR_REG_WORD,
        ops::MUL_ADDR_NUM_BYTE => names::full::MUL_ADDR_NUM_BYTE,
        ops::MUL_ADDR_NUM_WORD => names::full::MUL_ADDR_NUM_WORD,
        ops::MUL_ADDR_ADDR_BYTE => names::full::MUL_ADDR_ADDR_BYTE,
        ops::MUL_ADDR_ADDR_WORD => names::full::MUL_ADDR_ADDR_WORD,
        ops::MULS_REG_REG_BYTE => names::full::MULS_REG_REG_BYTE,
        ops::MULS_REG_REG_WORD => names::full::MULS_REG_REG_WORD,
        ops::MULS_REG_NUM_BYTE => names::full::MULS_REG_NUM_BYTE,
        ops::MULS_REG_NUM_WORD => names::full::MULS_REG_NUM_WORD,
        ops::MULS_REG_ADDR_BYTE => names::full::MULS_REG_ADDR_BYTE,
        ops::MULS_REG_ADDR_WORD => names::full::MULS_REG_ADDR_WORD,
        ops::MULS_ADDR_REG_BYTE => names::full::MULS_ADDR_REG_BYTE,
        ops::MULS_ADDR_REG_WORD => names::full::MULS_ADDR_REG_WORD,
        ops::MULS_ADDR_NUM_BYTE => names::full::MULS_ADDR_NUM_BYTE,
        ops::MULS_ADDR_NUM_WORD => names::full::MULS_ADDR_NUM_WORD,
        ops::MULS_ADDR_ADDR_BYTE => names::full::MULS_ADDR_ADDR_BYTE,
        ops::MULS_ADDR_ADDR_WORD => names::full::MULS_ADDR_ADDR_WORD,
        ops::DIV_REG_REG_BYTE => names::full::DIV_REG_REG_BYTE,
        ops::DIV_REG_REG_WORD => names::full::DIV_REG_REG_WORD,
        ops::DIV_REG_NUM_BYTE => names::full::DIV_REG_NUM_BYTE,
        ops::DIV_REG_NUM_WORD => names::full::DIV_REG_NUM_WORD,
        ops::DIV_REG_ADDR_BYTE => names::full::DIV_REG_ADDR_BYTE,
        ops::DIV_REG_ADDR_WORD => names::full::DIV_REG_ADDR_WORD,
        ops::DIV_ADDR_REG_BYTE => names::full::DIV_ADDR_REG_BYTE,
        ops::DIV_ADDR_REG_WORD => names::full::DIV_ADDR_REG_WORD,
        ops::DIV_ADDR_NUM_BYTE => names::full::DIV_ADDR_NUM_BYTE,
        ops::DIV_ADDR_NUM_WORD => names::full::DIV_ADDR_NUM_WORD,
        ops::DIV_ADDR_ADDR_BYTE => names::full::DIV_ADDR_ADDR_BYTE,
        ops::DIV_ADDR_ADDR_WORD => names::full::DIV_ADDR_ADDR_WORD,
        ops::DIVS_REG_REG_BYTE => names::full::DIVS_REG_REG_BYTE,
        ops::DIVS_REG_NUM_BYTE => names::full::DIVS_REG_NUM_BYTE,
        ops::DIVS_REG_ADDR_BYTE => names::full::DIVS_REG_ADDR_BYTE,
        ops::DIVS_ADDR_REG_BYTE => names::full::DIVS_ADDR_REG_BYTE,
        ops::DIVS_REG_REG_WORD => names::full::DIVS_REG_REG_WORD,
        ops::DIVS_REG_NUM_WORD => names::full::DIVS_REG_NUM_WORD,
        ops::DIVS_REG_ADDR_WORD => names::full::DIVS_REG_ADDR_WORD,
        ops::DIVS_ADDR_REG_WORD => names::full::DIVS_ADDR_REG_WORD,
        ops::DIVS_ADDR_NUM_BYTE => names::full::DIVS_ADDR_NUM_BYTE,
        ops::DIVS_ADDR_NUM_WORD => names::full::DIVS_ADDR_NUM_WORD,
        ops::DIVS_ADDR_ADDR_BYTE => names::full::DIVS_ADDR_ADDR_BYTE,
        ops::DIVS_ADDR_ADDR_WORD => names::full::DIVS_ADDR_ADDR_WORD,
        ops::JMP_REG => names::full::JMP_REG,
        ops::JMP_ADDR => names::full::JMP_ADDR,
        ops::JL_REG => names::full::JL_REG,
        ops::JL_ADDR => names::full::JL_ADDR,
        ops::JLE_REG => names::full::JLE_REG,
        ops::JLE_ADDR => names::full::JLE_ADDR,
        ops::JG_REG => names::full::JG_REG,
        ops::JG_ADDR => names::full::JG_ADDR,
        ops::JGE_REG => names::full::JGE_REG,
        ops::JGE_ADDR => names::full::JGE_ADDR,
        ops::JE_REG => names::full::JE_REG,
        ops::JE_ADDR => names::full::JE_ADDR,
        ops::JNE_REG => names::full::JNE_REG,
        ops::JNE_ADDR => names::full::JNE_ADDR,
        ops::CMP_REG_NUM_BYTE => names::full::CMP_REG_NUM_BYTE,
        ops::CMP_REG_NUM_WORD => names::full::CMP_REG_NUM_WORD,
        ops::CMP_REG_REG_BYTE => names::full::CMP_REG_REG_BYTE,
        ops::CMP_REG_REG_WORD => names::full::CMP_REG_REG_WORD,
        ops::CMPS_REG_NUM_BYTE => names::full::CMPS_REG_NUM_BYTE,
        ops::CMPS_REG_NUM_WORD => names::full::CMPS_REG_NUM_WORD,
        ops::CMPS_REG_REG_BYTE => names::full::CMPS_REG_REG_BYTE,
        ops::CMPS_REG_REG_WORD => names::full::CMPS_REG_REG_WORD,
        ops::JBS_REG_REG => names::full::JBS_REG_REG,
        ops::JBS_REG_NUM => names::full::JBS_REG_NUM,
        ops::JBS_ADDR_REG => names::full::JBS_ADDR_REG,
        ops::JBS_ADDR_NUM => names::full::JBS_ADDR_NUM,
        ops::JBC_REG_REG => names::full::JBC_REG_REG,
        ops::JBC_REG_NUM => names::full::JBC_REG_NUM,
        ops::JBC_ADDR_REG => names::full::JBC_ADDR_REG,
        ops::JBC_ADDR_NUM => names::full::JBC_ADDR_NUM,
        ops::CMP_REG_ADDR_BYTE => names::full::CMP_REG_ADDR_BYTE,
        ops::CMP_REG_ADDR_WORD => names::full::CMP_REG_ADDR_WORD,
        ops::CMPS_REG_ADDR_BYTE => names::full::CMPS_REG_ADDR_BYTE,
        ops::CMPS_REG_ADDR_WORD => names::full::CMPS_REG_ADDR_WORD,
        ops::JRF_BYTE => names::full::JRF_BYTE,
        ops::JRB_BYTE => names::full::JRB_BYTE,
        ops::ADDC_REG_REG_BYTE => names::full::ADDC_REG_REG_BYTE,
        ops::ADDC_REG_REG_WORD => names::full::ADDC_REG_REG_WORD,
        ops::ADDC_REG_NUM_BYTE => names::full::ADDC_REG_NUM_BYTE,
        ops::ADDC_REG_NUM_WORD => names::full::ADDC_REG_NUM_WORD,
        ops::ADDC_REG_ADDR_BYTE => names::full::ADDC_REG_ADDR_BYTE,
        ops::ADDC_REG_ADDR_WORD => names::full::ADDC_REG_ADDR_WORD,
        ops::ADDC_ADDR_REG_BYTE => names::full::ADDC_ADDR_REG_BYTE,
        ops::ADDC_ADDR_REG_WORD => names::full::ADDC_ADDR_REG_WORD,
        ops::ADDC_ADDR_NUM_BYTE => names::full::ADDC_ADDR_NUM_BYTE,
        ops::ADDC_ADDR_NUM_WORD => names::full::ADDC_ADDR_NUM_WORD,
        ops::ADDC_ADDR_ADDR_BYTE => names::full::ADDC_ADDR_ADDR_BYTE,
        ops::ADDC_ADDR_ADDR_WORD => names::full::ADDC_ADDR_ADDR_WORD,
        ops::SUBC_REG_REG_BYTE => names::full::SUBC_REG_REG_BYTE,
        ops::SUBC_REG_REG_WORD => names::full::SUBC_REG_REG_WORD,
        ops::SUBC_REG_NUM_BYTE => names::full::SUBC_REG_NUM_BYTE,
        ops::SUBC_REG_NUM_WORD => names::full::SUBC_REG_NUM_WORD,
        ops::SUBC_REG_ADDR_BYTE => names::full::SUBC_REG_ADDR_BYTE,
        ops::SUBC_REG_ADDR_WORD => names::full::SUBC_REG_ADDR_WORD,
        ops::SUBC_ADDR_REG_BYTE => names::full::SUBC_ADDR_REG_BYTE,
        ops::SUBC_ADDR_REG_WORD => names::full::SUBC_ADDR_REG_WORD,
        ops::SUBC_ADDR_NUM_BYTE => names::full::SUBC_ADDR_NUM_BYTE,
        ops::SUBC_ADDR_NUM_WORD => names::full::SUBC_ADDR_NUM_WORD,
        ops::SUBC_ADDR_ADDR_BYTE => names::full::SUBC_ADDR_ADDR_BYTE,
        ops::SUBC_ADDR_ADDR_WORD => names::full::SUBC_ADDR_ADDR_WORD,
        ops::SLEEP => names::full::SLEEP,
        ops::PUSH_REG_BYTE => names::full::PUSH_REG_BYTE,
        ops::PUSH_REG_WORD => names::full::PUSH_REG_WORD,
        ops::PUSH_NUM_BYTE => names::full::PUSH_NUM_BYTE,
        ops::PUSH_NUM_WORD => names::full::PUSH_NUM_WORD,
        ops::POP_REG_BYTE => names::full::POP_REG_BYTE,
        ops::POP_REG_WORD => names::full::POP_REG_WORD,
        _ => return Err(InvalidInstructionCode(op)),
    };
    Ok(name)
}

#[cfg(test)]
mod test {
    use crate::ops::ALL;
    use crate::{op_desc, op_name};

    #[test]
    fn check_have_names() {
        for op in ALL {
            assert!(op_name(op).is_ok(), "{:02X}", op);
            assert!(op_desc(op).is_ok(), "{:02X}", op);
        }
    }
}
