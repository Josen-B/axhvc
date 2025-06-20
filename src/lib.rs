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
        HyperVisorDebug = 2,
        /// Only for debugging purposes.
        HDebug = HYPER_CALL_CODE_PRIVILEGED_MASK | 0,
        /// Create a new instance, pass the raw binary/executable file by shared pages.
        HCreateInstance = HYPER_CALL_CODE_PRIVILEGED_MASK | 1,
        /// Exit from a insance process.
        HExitProcess = HYPER_CALL_CODE_PRIVILEGED_MASK | 2,
        /// Exit from a instance, this is called by the instance when the last process in the instance exits.
        HShutdownInstance = HYPER_CALL_CODE_PRIVILEGED_MASK | 3,
        HMMAP = HYPER_CALL_CODE_PRIVILEGED_MASK | 4,
        /// Clone current gaddrspace to a new one, return its EPTP list index.
        HClone = HYPER_CALL_CODE_PRIVILEGED_MASK | 5,
        // Init ring 0 shim.
        HInitShim = HYPER_CALL_CODE_PRIVILEGED_MASK | 6,
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
            HyperCallCode::HyperVisorDebug => write!(f, "HyperVisorDebug {:#x}", *self as u32),
            HyperCallCode::HDebug => write!(f, "HDebug {:#x}", *self as u32),
            HyperCallCode::HRead => write!(f, "HRead {:#x}", *self as u32),
            HyperCallCode::HWrite => write!(f, "HWrite {:#x}", *self as u32),
            HyperCallCode::HCreateInstance => write!(f, "HCreateInstance {:#x}", *self as u32),
            HyperCallCode::HExitProcess => write!(f, "HExitProcess {:#x}", *self as u32),
            HyperCallCode::HShutdownInstance => write!(f, "HShutdownInstance {:#x}", *self as u32),
            HyperCallCode::HMMAP => write!(f, "HMMAP {:#x}", *self as u32),
            HyperCallCode::HClone => write!(f, "HClone {:#x}", *self as u32),
            HyperCallCode::HInitShim => write!(f, "HInitShim {:#x}", *self as u32),
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
