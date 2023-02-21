use crate::drivers::pci::pci::*;
use crate::drivers::usb::usb::*;

#[derive(Debug)]
pub struct CapabilityRegisters {
    CapabilityRegistersLength: u8, // ReadOnly CAPLENGTH
    Reserved1: u8,
    HostControllerInterfaceVersionNumber: u16, // ReadOnly HCIVERSION
    StructuralParameters1: u32,                // ReadOnly HCSPARAMS1
    StructuralParameters2: u32,                // ReadOnly HCSPARAMS2
    StructuralParameters3: u32,                // ReadOnly HCSPARAMS3
    CapabilityParameters1: u32,                // ReadOnly HCCPARAMS1
    DoorbellOffset: u32,                       // ReadOnly DBOFF
    RuntimeRegisterSpaceOffset: u32,           // ReadOnly RTSOFF
    CapabilityParameters2: u32,                // ReadOnly HCCPARAMS2
    VirtualizationBasedTrustedIORegisterSpaceOffset: u32, // Readonly VTIOSOFF
}

#[derive(Debug)]
pub struct Xhci {
    PCIDevice: PCIDevice,
    CapabilityRegisters: CapabilityRegisters,
    MemoryMappedIOBaseAddress: u64,
}

impl Xhci {
    pub fn new(pci_device: PCIDevice) -> Self {
        // let mmio_base: u64 =
        //     pci_device.pci_configuration.base_address_register0 & 0xFFFFFFF0 | (BAR1 << 32);

        // Xhci {
        //     PCIDevice: pci_device,
        //     MemoryMappedIOBaseAddress: mmio_base,
        // }
        todo!()
    }

    pub fn switch_ehci_to_xhci(&mut self) -> Result<(), USBDriverError> {
        todo!()
    }
}
