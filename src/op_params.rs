use crate::op_params::ID::{Dec, Inc};
use crate::op_params::PP::{Post, Pre};
use crate::LangError;
use crate::LangError::InvalidRegisterArgCode;

pub const MASK: u8 = 0xF0;

pub const REGISTER: u8 = 0b00000000;
//pub const RESERVED: u8 =   0b00010000;
//pub const RESERVED: u8 =   0b00100000;
//pub const RESERVED: u8 =   0b00110000;
pub const POST_INC: u8 = 0b01000000;
pub const POST_DEC: u8 = 0b01010000;
pub const PRE_INC: u8 = 0b01100000;
pub const PRE_DEC: u8 = 0b01110000;
pub const INDIRECT: u8 = 0b10000000;
pub const IND_OFFSET_REG: u8 = 0b10010000;
pub const IND_OFFSET_NUM: u8 = 0b10100000;
//pub const RESERVED: u8 =   0b10110000;
pub const IND_POST_INC: u8 = 0b11000000;
pub const IND_POST_DEC: u8 = 0b11010000;
pub const IND_PRE_INC: u8 = 0b11100000;
pub const IND_PRE_DEC: u8 = 0b11110000;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PP {
    Pre,
    Post,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ID {
    Inc,
    Dec,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RegisterPPID {
    pub is_indirect: bool,
    pub is_offset_reg: bool,
    pub is_offset_num: bool,
    pub ppid: Option<(PP, ID)>,
}

impl RegisterPPID {
    pub fn new(
        is_indirect: bool,
        is_offset_reg: bool,
        is_offset_num: bool,
        ppid: Option<(PP, ID)>,
    ) -> Self {
        Self {
            is_indirect,
            is_offset_reg,
            is_offset_num,
            ppid,
        }
    }
}

impl TryFrom<u8> for RegisterPPID {
    type Error = LangError;

    fn try_from(value: u8) -> Result<Self, LangError> {
        let ppid = match value & MASK {
            IND_PRE_DEC => RegisterPPID::new(true, false, false, Some((Pre, Dec))),
            IND_POST_DEC => RegisterPPID::new(true, false, false, Some((Post, Dec))),
            IND_PRE_INC => RegisterPPID::new(true, false, false, Some((Pre, Inc))),
            IND_POST_INC => RegisterPPID::new(true, false, false, Some((Post, Inc))),
            INDIRECT => RegisterPPID::new(true, false, false, None),
            PRE_DEC => RegisterPPID::new(false, false, false, Some((Pre, Dec))),
            POST_DEC => RegisterPPID::new(false, false, false, Some((Post, Dec))),
            PRE_INC => RegisterPPID::new(false, false, false, Some((Pre, Inc))),
            POST_INC => RegisterPPID::new(false, false, false, Some((Post, Inc))),
            REGISTER => RegisterPPID::new(false, false, false, None),
            IND_OFFSET_REG => RegisterPPID::new(true, true, false, None),
            IND_OFFSET_NUM => RegisterPPID::new(true, false, true, None),
            _ => return Err(InvalidRegisterArgCode(value)),
        };
        Ok(ppid)
    }
}

impl From<RegisterPPID> for u8 {
    fn from(arg: RegisterPPID) -> Self {
        let mut byte = 0;

        if arg.is_offset_reg {
            return IND_OFFSET_REG;
        } else if arg.is_offset_num {
            return IND_OFFSET_NUM;
        } else if arg.is_indirect {
            byte |= INDIRECT;
        }
        if let Some(ppid) = arg.ppid {
            match (ppid.0, ppid.1) {
                (Pre, Inc) => byte |= PRE_INC,
                (Post, Inc) => byte |= POST_INC,
                (Pre, Dec) => byte |= PRE_DEC,
                (Post, Dec) => byte |= POST_DEC,
            }
        }

        byte
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_alignment() {
        let values = vec![
            REGISTER,
            POST_INC,
            POST_DEC,
            PRE_INC,
            PRE_DEC,
            INDIRECT,
            IND_POST_INC,
            IND_POST_DEC,
            IND_PRE_INC,
            IND_PRE_DEC,
            IND_OFFSET_REG,
            IND_OFFSET_NUM,
        ];
        for value in values {
            let ppid: RegisterPPID = value.try_into().unwrap();
            let byte: u8 = ppid.into();
            assert_eq!(value, byte, "{}", value);
        }
    }
}
