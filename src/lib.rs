#![no_std]

use bit_field::BitField;
use numeric_enum_macro::numeric_enum;

use axerrno::AxResult;

const HYPER_CALL_CODE_PRIVILEGED_MASK: u32 = 0xe000_0000;

numeric_enum! {
    #[repr(u32)]
    #[derive(Eq, PartialEq, Copy, Clone)]
    pub enum HyperCallCode {
        /// Disable the hypervisor.
        HypervisorDisable = 0,
        /// Prepare to disable the hypervisor, map the hypervisor memory to the guest.
        HyperVisorPrepareDisable = 1,
        /// Only for debugging purposes.
        HDebug = HYPER_CALL_CODE_PRIVILEGED_MASK | 0,
        /// Create a new instance, including the first gate instance.
        HCreateInstance = HYPER_CALL_CODE_PRIVILEGED_MASK | 1,
        /// Exit from a insance.
        HExitInstance = HYPER_CALL_CODE_PRIVILEGED_MASK | 2,
        HMMAP = HYPER_CALL_CODE_PRIVILEGED_MASK | 4,
        HClone = HYPER_CALL_CODE_PRIVILEGED_MASK | 5,
        /// Only for debugging purposes, console read.
        HRead = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x11,
        /// Only for debugging purposes, console write.
        HWrite = HYPER_CALL_CODE_PRIVILEGED_MASK | 0x12,
    }
}

impl core::fmt::Debug for HyperCallCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(")?;
        match self {
            HyperCallCode::HypervisorDisable => write!(f, "HypervisorDisable {:#x}", *self as u32),
            HyperCallCode::HyperVisorPrepareDisable => {
                write!(f, "HyperVisorPrepareDisable {:#x}", *self as u32)
            }
            HyperCallCode::HDebug => write!(f, "HDebug {:#x}", *self as u32),
            HyperCallCode::HRead => write!(f, "HRead {:#x}", *self as u32),
            HyperCallCode::HWrite => write!(f, "HWrite {:#x}", *self as u32),
            HyperCallCode::HCreateInstance => write!(f, "HCreateInstance {:#x}", *self as u32),
            HyperCallCode::HExitInstance => write!(f, "HExitInstance {:#x}", *self as u32),
            HyperCallCode::HMMAP => write!(f, "HMMAP {:#x}", *self as u32),
            HyperCallCode::HClone => write!(f, "HClone {:#x}", *self as u32),
        }?;
        write!(f, ")")
    }
}

impl HyperCallCode {
    pub fn is_privileged(self) -> bool {
        (self as u32).get_bits(29..32) == 0
    }
}

pub type HyperCallResult = AxResult<usize>;
