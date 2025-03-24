use log::debug;
use axerrno::AxResult;

numeric_enum_macro::numeric_enum! {
    #[repr(usize)]
    #[allow(non_camel_case_types)]
    #[allow(missing_docs)]
    #[derive(Eq, PartialEq, Debug, Copy, Clone)]
    pub enum HyperCallId {
        HV_ENABLE = 0,
        HV_DISABLE = 1,
        VM_LIST = 2,
        VM_CREATE= 3,
        VM_BOOT = 4,
        VM_SHUTDOWN = 5,
    }
}

impl TryFrom<u64> for HyperCallId {
    type Error = &'static str;

    fn try_from(nr: u64) -> Result<Self, Self::Error> {
        match nr {
            0 => Ok(HyperCallId::HV_ENABLE),
            1 => Ok(HyperCallId::HV_DISABLE),
            2 => Ok(HyperCallId::VM_LIST),
            3 => Ok(HyperCallId::VM_CREATE),
            4 => Ok(HyperCallId::VM_BOOT),
            5 => Ok(HyperCallId::VM_SHUTDOWN),
            _ => Err("Unsupported hypercall id"),
        }
    }
}

pub fn hypercall(hypercall_id: HyperCallId, args: [u64; 6]) -> AxResult {
    debug!("hypercall: id={:?}, args={:x?}", hypercall_id, args);
    match hypercall_id {
        HyperCallId::HV_ENABLE => todo!(),
        HyperCallId::HV_DISABLE => todo!(),
        HyperCallId::VM_LIST => todo!(),
        HyperCallId::VM_CREATE => todo!(),
        HyperCallId::VM_BOOT => todo!(),
        HyperCallId::VM_SHUTDOWN => todo!(),
    }
}