//！ AxVisor HyperCall definitions
#![no_std]

use numeric_enum_macro::numeric_enum;

use axerrno::AxResult;

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
        /// Request to setup a IVC shared memory channel as a publisher.
        /// Arguments:
        /// - `key`: The key of the IVC channel.
        /// - `shm_base_gpa_ptr`: The pointer to the base address of the shared memory region in guest physical address.
        /// - `shm_size_ptr`: The pointer to the size of the shared memory region in guest physical address.
        HIVCPublishChannel = 3,
        /// Request to subscriber a IVC shared memory channel.
        /// Arguments:
        /// - `publisher_vm_id`: The ID of the publisher VM.
        /// - `key`: The key of the IVC channel.
        /// - `shm_base_gpa_ptr`: The pointer to the base address of the shared memory region in guest physical address.
        /// - `shm_size_ptr`: The pointer to the size of the shared memory region in guest physical address.
        HIVCSubscribChannel = 4,
        /// Request to un-register a IVC shared memory channel as a publisher.
        /// Arguments:
        /// - `key`: The key of the IVC channel.
        HIVCUnPublishChannel = 5,
        /// Request to un-subscribe a IVC shared memory channel.
        /// Arguments:
        /// - `publisher_vm_id`: The ID of the publisher VM.
        /// - `key`: The key of the IVC channel.
        HIVCUnSubscribChannel = 6,
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
            HyperCallCode::HIVCPublishChannel => {
                write!(f, "HIVCPublishChannel {:#x}", *self as u32)
            }
            HyperCallCode::HIVCSubscribChannel => {
                write!(f, "HIVCSubscribChannel {:#x}", *self as u32)
            }
            HyperCallCode::HIVCUnPublishChannel => {
                write!(f, "HIVCUnPublishChannel {:#x}", *self as u32)
            }
            HyperCallCode::HIVCUnSubscribChannel => {
                write!(f, "HIVCUnSubscribChannel {:#x}", *self as u32)
            }
        }?;
        write!(f, ")")
    }
}

pub type HyperCallResult = AxResult<usize>;
