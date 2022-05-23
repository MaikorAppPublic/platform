pub mod names {
    pub const AH: &str = "AH";
    pub const AL: &str = "AL";
    pub const BH: &str = "BH";
    pub const BL: &str = "BL";
    pub const CH: &str = "CH";
    pub const CL: &str = "CL";
    pub const DH: &str = "DH";
    pub const DL: &str = "DL";
    pub const AX: &str = "AX";
    pub const BX: &str = "BX";
    pub const CX: &str = "CX";
    pub const DX: &str = "DX";
    pub const FLAGS: &str = "FLG";

    pub const ALL: [&str; 13] = [AH, AL, BH, BL, CH, CL, DH, DL, AX, BX, CX, DX, FLAGS];
}

/// ID of register, this should be combined with op_params
/// for op args
pub mod id {
    use crate::registers::names;
    use crate::LangError;
    use crate::LangError::{InvalidRegisterId, InvalidRegisterName};

    pub const AH: u8 = 0;
    pub const AL: u8 = 1;
    pub const BH: u8 = 2;
    pub const BL: u8 = 3;
    pub const CH: u8 = 4;
    pub const CL: u8 = 5;
    pub const DH: u8 = 6;
    pub const DL: u8 = 7;
    pub const FLAGS: u8 = 8;
    pub const AX: u8 = 9;
    pub const BX: u8 = 10;
    pub const CX: u8 = 11;
    pub const DX: u8 = 12;

    pub const fn size(value: u8) -> usize {
        if matches!(value, AX | BX | CX | DX) {
            2
        } else {
            1
        }
    }

    pub fn to_name(value: u8) -> Result<&'static str, LangError> {
        let name = match value as usize {
            AH => names::AH,
            AL => names::AL,
            BH => names::BH,
            BL => names::BL,
            CH => names::CH,
            CL => names::CL,
            DH => names::DH,
            DL => names::DL,
            AX => names::AX,
            BX => names::BX,
            CX => names::CX,
            DX => names::DX,
            FLAGS => names::FLAGS,
            _ => return Err(InvalidRegisterId(value)),
        };
        Ok(name)
    }

    pub fn from_name(value: &str) -> Result<usize, LangError> {
        let code = match value.to_ascii_uppercase().as_str() {
            names::AH => AH,
            names::AL => AL,
            names::BH => BH,
            names::BL => BL,
            names::CH => CH,
            names::CL => CL,
            names::DH => DH,
            names::DL => DL,
            names::AX => AX,
            names::BX => BX,
            names::CX => CX,
            names::DX => DX,
            names::FLAGS => FLAGS,
            _ => return Err(InvalidRegisterName(value.to_string())),
        };
        Ok(code)
    }

    pub const ALL: [usize; 13] = [AH, AL, BH, BL, CH, CL, DH, DL, AX, BX, CX, DX, FLAGS];
}

pub const SIZE: usize = 9;

pub mod flags {
    pub const CARRY: u8 = 0b10000000;
    pub const ZERO: u8 = 0b01000000;
    pub const SIGNED: u8 = 0b00100000;
    pub const OVERFLOW: u8 = 0b00010000;
    pub const LESS_THAN: u8 = 0b00001000;
    pub const GREATER_THAN: u8 = 0b00000100;
    //pub const RESERVED: u8 = 0b00000010;
    pub const INTERRUPTS: u8 = 0b00000001;
}

pub const FLG_DEFAULT: u8 = flags::ZERO + flags::INTERRUPTS;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_ids_align() {
        assert_eq!(id::to_name(id::AH as u8).unwrap(), names::AH);
        assert_eq!(id::to_name(id::AL as u8).unwrap(), names::AL);
        assert_eq!(id::to_name(id::BH as u8).unwrap(), names::BH);
        assert_eq!(id::to_name(id::BL as u8).unwrap(), names::BL);
        assert_eq!(id::to_name(id::CH as u8).unwrap(), names::CH);
        assert_eq!(id::to_name(id::CL as u8).unwrap(), names::CL);
        assert_eq!(id::to_name(id::DH as u8).unwrap(), names::DH);
        assert_eq!(id::to_name(id::DL as u8).unwrap(), names::DL);
        assert_eq!(id::to_name(id::AX as u8).unwrap(), names::AX);
        assert_eq!(id::to_name(id::BX as u8).unwrap(), names::BX);
        assert_eq!(id::to_name(id::CX as u8).unwrap(), names::CX);
        assert_eq!(id::to_name(id::DX as u8).unwrap(), names::DX);
        assert_eq!(id::to_name(id::FLAGS as u8).unwrap(), names::FLAGS);

        assert_eq!(id::from_name(names::AH).unwrap(), id::AH);
        assert_eq!(id::from_name(names::AL).unwrap(), id::AL);
        assert_eq!(id::from_name(names::BH).unwrap(), id::BH);
        assert_eq!(id::from_name(names::BL).unwrap(), id::BL);
        assert_eq!(id::from_name(names::CH).unwrap(), id::CH);
        assert_eq!(id::from_name(names::CL).unwrap(), id::CL);
        assert_eq!(id::from_name(names::DH).unwrap(), id::DH);
        assert_eq!(id::from_name(names::DL).unwrap(), id::DL);
        assert_eq!(id::from_name(names::AX).unwrap(), id::AX);
        assert_eq!(id::from_name(names::BX).unwrap(), id::BX);
        assert_eq!(id::from_name(names::CX).unwrap(), id::CX);
        assert_eq!(id::from_name(names::DX).unwrap(), id::DX);
        assert_eq!(id::from_name(names::FLAGS).unwrap(), id::FLAGS);
    }

    #[test]
    fn check_invalid_values() {
        assert!(id::from_name("gsgad").is_err());
        assert!(id::to_name(255).is_err());
    }
}
