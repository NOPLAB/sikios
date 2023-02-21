use crate::drivers::pci::device_info::*;
use core::{
    panic,
    str::{self},
};
use ux::u24;
use x86_64::instructions::port::{Port, PortWriteOnly};

const ConfigAddress: u16 = 0x0cf8;
const ConfigData: u16 = 0x0cfc;

#[derive(Debug, Copy, Clone, Default)]
pub struct PCIDeviceType {
    pci_code_class: PCIClassCode,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct PCIDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub pci_configuration: PCIConfiguration,
    pub device_type: PCIDeviceType,
}

impl PCIDevice {}

#[derive(Debug, Copy, Clone, Default)]
pub struct PCIConfiguration {
    pub vendor_id: u16,
    pub device_id: u16,

    pub command: u16,
    pub status: u16,

    pub revision_id: u8,
    pub interface: u8,
    pub sub_class: u8,
    pub base_class: u8,

    pub chacheline_size: u8,
    pub latency_timer: u8,
    pub header_type: u8,
    pub bist: u8,

    pub base_address_register0: u32,

    pub base_address_register1: u32,

    pub base_address_register2: u32,

    pub base_address_register3: u32,

    pub base_address_register4: u32,

    pub base_address_register5: u32,

    pub cardbus_cis_pointer: u32,

    pub subsystem_vendor_id: u16,
    pub subsystem_id: u16,

    pub expansion_rom_base_address: u32,

    pub capabilities_pointer: u8,
    reserved0: Option<u24>,

    reserved1: Option<u32>,

    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub min_gnt: u8,
    pub max_lat: u8,
}

impl PCIConfiguration {
    pub fn is_single_function_device(self) -> bool {
        self.header_type & 0x80 == 0
    }

    pub fn return_device_type(self) -> PCIDeviceType {
        let class_code = match self.base_class {
            0x0 => match self.sub_class {
                0x0 => PCIClassCode::Unclassified(Unclassified::NonVGACompatibleUnclassifiedDevice),
                0x1 => PCIClassCode::Unclassified(Unclassified::VGACompatibleUnclassifiedDevice),
                _ => panic!(),
            },
            0x1 => match self.sub_class {
                0x0 => {
                    PCIClassCode::MassStorageController(MassStorageController::SCSIBusController)
                }
                0x1 => PCIClassCode::MassStorageController(MassStorageController::IDEController),
                0x2 => {
                    PCIClassCode::MassStorageController(MassStorageController::FloppyDiskController)
                }
                0x3 => PCIClassCode::MassStorageController(MassStorageController::IPIBusController),
                0x4 => PCIClassCode::MassStorageController(MassStorageController::RAIDController),
                0x5 => PCIClassCode::MassStorageController(MassStorageController::ATAController),
                0x6 => {
                    PCIClassCode::MassStorageController(MassStorageController::SerialATAController)
                }
                0x7 => PCIClassCode::MassStorageController(
                    MassStorageController::SerialAttachedSCSIController,
                ),
                0x8 => PCIClassCode::MassStorageController(
                    MassStorageController::NonVolatileMemoryController,
                ),
                0x80 => PCIClassCode::MassStorageController(MassStorageController::Other),
                _ => panic!(),
            },
            0x2 => match self.sub_class {
                0x0 => PCIClassCode::NetworkController(NetworkController::EthernetController),
                0x1 => PCIClassCode::NetworkController(NetworkController::TokenRingController),
                0x2 => PCIClassCode::NetworkController(NetworkController::FDDIController),
                0x3 => PCIClassCode::NetworkController(NetworkController::ATMController),
                0x4 => PCIClassCode::NetworkController(NetworkController::ISDNController),
                0x5 => PCIClassCode::NetworkController(NetworkController::WorldFipController),
                0x6 => PCIClassCode::NetworkController(
                    NetworkController::PICMG214MultiComputingController,
                ),
                0x7 => PCIClassCode::NetworkController(NetworkController::InfinibandController),
                0x8 => PCIClassCode::NetworkController(NetworkController::FabricController),
                0x80 => PCIClassCode::NetworkController(NetworkController::Other),
                _ => panic!(),
            },
            0x3 => match self.sub_class {
                0x0 => PCIClassCode::DisplayController(DisplayController::VGACompatibleController),
                0x1 => PCIClassCode::DisplayController(DisplayController::XGAController),
                0x2 => PCIClassCode::DisplayController(DisplayController::_3DController),
                0x80 => PCIClassCode::DisplayController(DisplayController::Other),
                _ => panic!(),
            },
            0x4 => match self.sub_class {
                0x0 => PCIClassCode::MultimediaController(
                    MultimediaController::MultimediaVideoController,
                ),
                0x1 => PCIClassCode::MultimediaController(
                    MultimediaController::MultimediaAudioController,
                ),
                0x2 => PCIClassCode::MultimediaController(
                    MultimediaController::ComputerTelephonyDevice,
                ),
                0x3 => PCIClassCode::MultimediaController(MultimediaController::AudioDevice),
                0x80 => PCIClassCode::MultimediaController(MultimediaController::Other),
                _ => panic!(),
            },
            0x5 => match self.sub_class {
                0x0 => PCIClassCode::MemoryController(MemoryController::RAMController),
                0x1 => PCIClassCode::MemoryController(MemoryController::FlashController),
                0x80 => PCIClassCode::MemoryController(MemoryController::Other),
                _ => panic!(),
            },
            0x6 => match self.sub_class {
                0x0 => PCIClassCode::Bridge(Bridge::HostBridge),
                0x1 => PCIClassCode::Bridge(Bridge::ISABridge),
                0x2 => PCIClassCode::Bridge(Bridge::EISABridge),
                0x3 => PCIClassCode::Bridge(Bridge::MCABridge),
                0x4 => PCIClassCode::Bridge(Bridge::PCItoPCIBridge1),
                0x5 => PCIClassCode::Bridge(Bridge::PCMCIABridge),
                0x6 => PCIClassCode::Bridge(Bridge::NuBusBridge),
                0x7 => PCIClassCode::Bridge(Bridge::CardBusBridge),
                0x8 => PCIClassCode::Bridge(Bridge::RACEwayBridge),
                0x9 => PCIClassCode::Bridge(Bridge::PCItoPCIBridge2),
                0x0A => PCIClassCode::Bridge(Bridge::InfiniBandtoPCIHostBridge),
                0x80 => PCIClassCode::Bridge(Bridge::Other),
                _ => panic!(),
            },
            0x7 => match self.sub_class {
                0x0 => PCIClassCode::SimpleCommunicationController(
                    SimpleCommunicationController::SerialController,
                ),
                0x1 => PCIClassCode::SimpleCommunicationController(
                    SimpleCommunicationController::ParallelController,
                ),
                0x2 => PCIClassCode::SimpleCommunicationController(
                    SimpleCommunicationController::MultiportSerialController,
                ),
                0x3 => PCIClassCode::SimpleCommunicationController(
                    SimpleCommunicationController::Modem,
                ),
                0x4 => PCIClassCode::SimpleCommunicationController(
                    SimpleCommunicationController::IEEE48812Controller,
                ),
                0x5 => PCIClassCode::SimpleCommunicationController(
                    SimpleCommunicationController::SmartCardController,
                ),
                0x80 => PCIClassCode::SimpleCommunicationController(
                    SimpleCommunicationController::Other,
                ),
                _ => panic!(),
            },
            0x8 => match self.sub_class {
                0x0 => PCIClassCode::BaseSystemPeripheral(BaseSystemPeripheral::PIC),
                0x1 => PCIClassCode::BaseSystemPeripheral(BaseSystemPeripheral::DMAConrtoller),
                0x2 => PCIClassCode::BaseSystemPeripheral(BaseSystemPeripheral::Timer),
                0x3 => PCIClassCode::BaseSystemPeripheral(BaseSystemPeripheral::RTCController),
                0x4 => {
                    PCIClassCode::BaseSystemPeripheral(BaseSystemPeripheral::PCIHotPlugController)
                }
                0x5 => PCIClassCode::BaseSystemPeripheral(BaseSystemPeripheral::SDHostController),
                0x6 => PCIClassCode::BaseSystemPeripheral(BaseSystemPeripheral::IOMMU),
                0x80 => PCIClassCode::BaseSystemPeripheral(BaseSystemPeripheral::Other),
                _ => panic!(),
            },
            0x9 => match self.sub_class {
                0x0 => {
                    PCIClassCode::InputDeviceController(InputDeviceController::KeyboardController)
                }
                0x1 => PCIClassCode::InputDeviceController(InputDeviceController::DigitizerPen),
                0x2 => PCIClassCode::InputDeviceController(InputDeviceController::MouseController),
                0x3 => {
                    PCIClassCode::InputDeviceController(InputDeviceController::ScannerController)
                }
                0x4 => {
                    PCIClassCode::InputDeviceController(InputDeviceController::GameportController)
                }
                0x80 => PCIClassCode::InputDeviceController(InputDeviceController::Other),
                _ => panic!(),
            },
            0xA => match self.sub_class {
                0x0 => PCIClassCode::DockingStation(DockingStation::Generic),
                0x80 => PCIClassCode::DockingStation(DockingStation::Other),
                _ => panic!(),
            },
            0xB => match self.sub_class {
                0x0 => PCIClassCode::Processor(PCISubClassProcessor::_386),
                0x1 => PCIClassCode::Processor(PCISubClassProcessor::_486),
                0x2 => PCIClassCode::Processor(PCISubClassProcessor::Pentium),
                0x3 => PCIClassCode::Processor(PCISubClassProcessor::PentiumPro),
                0x10 => PCIClassCode::Processor(PCISubClassProcessor::Alpha),
                0x20 => PCIClassCode::Processor(PCISubClassProcessor::PowerPC),
                0x30 => PCIClassCode::Processor(PCISubClassProcessor::MIPS),
                0x40 => PCIClassCode::Processor(PCISubClassProcessor::CoProcessor),
                0x80 => PCIClassCode::Processor(PCISubClassProcessor::Other),
                _ => panic!(),
            },
            0xC => match self.sub_class {
                0x0 => PCIClassCode::SerialBusController(SerialBusController::FireWireController),
                0x1 => PCIClassCode::SerialBusController(SerialBusController::ACCESSBusController),
                0x2 => PCIClassCode::SerialBusController(SerialBusController::SSA),
                0x3 => match self.interface {
                    0x0 => PCIClassCode::SerialBusController(SerialBusController::USBController(
                        USBController::UHCIController,
                    )),
                    0x10 => PCIClassCode::SerialBusController(SerialBusController::USBController(
                        USBController::OHCIController,
                    )),
                    0x20 => PCIClassCode::SerialBusController(SerialBusController::USBController(
                        USBController::EHCIController,
                    )),
                    0x30 => PCIClassCode::SerialBusController(SerialBusController::USBController(
                        USBController::XHCIController,
                    )),
                    0x80 => PCIClassCode::SerialBusController(SerialBusController::USBController(
                        USBController::Unspecified,
                    )),
                    0xFE => PCIClassCode::SerialBusController(SerialBusController::USBController(
                        USBController::USBDevice,
                    )),
                    _ => panic!(),
                },
                0x4 => PCIClassCode::SerialBusController(SerialBusController::FibreChannel),
                0x5 => PCIClassCode::SerialBusController(SerialBusController::SMBusController),
                0x6 => PCIClassCode::SerialBusController(SerialBusController::InfiniBandController),
                0x7 => PCIClassCode::SerialBusController(SerialBusController::IPMIInterface),
                0x8 => PCIClassCode::SerialBusController(SerialBusController::SERCOSInterface),
                0x9 => PCIClassCode::SerialBusController(SerialBusController::CANbusController),
                0x80 => PCIClassCode::SerialBusController(SerialBusController::Other),
                _ => panic!(),
            },
            0xD => match self.sub_class {
                0x0 => PCIClassCode::WirelessController(
                    PCISubClassWirelessController::iRDACompatibleController,
                ),
                0x1 => PCIClassCode::WirelessController(
                    PCISubClassWirelessController::ConsumerIRController,
                ),
                0x10 => {
                    PCIClassCode::WirelessController(PCISubClassWirelessController::RFController)
                }
                0x11 => PCIClassCode::WirelessController(
                    PCISubClassWirelessController::BluetoothController,
                ),
                0x12 => PCIClassCode::WirelessController(
                    PCISubClassWirelessController::BroadbandController,
                ),
                0x20 => PCIClassCode::WirelessController(
                    PCISubClassWirelessController::EthernetController8021a,
                ),
                0x21 => PCIClassCode::WirelessController(
                    PCISubClassWirelessController::EthernetController8021b,
                ),
                0x80 => PCIClassCode::WirelessController(PCISubClassWirelessController::Other),
                _ => panic!(),
            },
            0xE => match self.sub_class {
                0x0 => PCIClassCode::IntelligentController(PCISubClassIntelligentController::I20),
                _ => panic!(),
            },
            0xF => match self.sub_class {
                0x1 => PCIClassCode::SatelliteCommunicationController(
                    SatelliteCommunicationController::SatelliteTVController,
                ),
                0x2 => PCIClassCode::SatelliteCommunicationController(
                    SatelliteCommunicationController::SatelliteAudioController,
                ),
                0x3 => PCIClassCode::SatelliteCommunicationController(
                    SatelliteCommunicationController::SatelliteVoiceController,
                ),
                0x4 => PCIClassCode::SatelliteCommunicationController(
                    SatelliteCommunicationController::SatelliteDataController,
                ),
                _ => panic!(),
            },
            0x10 => match self.sub_class {
                0x0 => PCIClassCode::EncryptionController(
                    EncryptionController::NetworkAndComputingEncryptionDecryption,
                ),
                0x10 => PCIClassCode::EncryptionController(
                    EncryptionController::EntertainmentEncryptionDecryption,
                ),
                0x80 => PCIClassCode::EncryptionController(EncryptionController::Other),
                _ => panic!(),
            },
            0x11 => match self.sub_class {
                0x0 => PCIClassCode::SignalProcessingController(
                    SignalProcessingController::DPIOModules,
                ),
                0x0 => PCIClassCode::SignalProcessingController(
                    SignalProcessingController::PerformanceCounters,
                ),
                0x0 => PCIClassCode::SignalProcessingController(
                    SignalProcessingController::CommunicationSynchronizer,
                ),
                0x0 => PCIClassCode::SignalProcessingController(
                    SignalProcessingController::SignalProcessingManagement,
                ),
                0x0 => PCIClassCode::SignalProcessingController(SignalProcessingController::Other),
                _ => panic!(),
            },
            0x12 => PCIClassCode::ProcessingAccelerator,
            0x13 => PCIClassCode::NonEssentialInstrumentation,
            0x14 => PCIClassCode::Reserved0x3F,
            0x40 => PCIClassCode::CoProcessor,
            0x41 => PCIClassCode::Reserved0xFE,
            0xFF => PCIClassCode::UnassignedClass,
            _ => panic!(),
        };

        PCIDeviceType {
            pci_code_class: class_code,
        }
    }
}

#[derive(Debug)]
pub struct PCI {
    address: PortWriteOnly<u32>,
    data: Port<u32>,
    pub devices: [PCIDevice; 256 * 32 * 8],
    pub device_index: usize,
}

impl PCI {
    pub fn new() -> Self {
        PCI {
            address: PortWriteOnly::new(ConfigAddress),
            data: Port::new(ConfigData),
            devices: [Default::default(); 256 * 32 * 8],
            device_index: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.scan_all_bus().unwrap();
    }

    fn config_address(bus: u8, device: u8, function: u8, register_address: u8) -> u32 {
        0b1000_0000_0000_0000_0000_0000_0000_0000
            | (bus as u32) << 16
            | (device as u32) << 11
            | (function as u32) << 8
            | (register_address as u32) & 0xfc
    }

    pub fn read_vendor_id(&mut self, bus: u8, device: u8, function: u8) -> u16 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x00));
            self.data.read() as u16
        }
    }

    pub fn read_device_id(&mut self, bus: u8, device: u8, function: u8) -> u16 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x00));
            (self.data.read() >> 16) as u16
        }
    }

    pub fn read_command(&mut self, bus: u8, device: u8, function: u8) -> u16 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x04));
            self.data.read() as u16
        }
    }

    pub fn read_status(&mut self, bus: u8, device: u8, function: u8) -> u16 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x04));
            (self.data.read() >> 16) as u16
        }
    }

    pub fn read_revision_id(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x08));
            self.data.read() as u8
        }
    }

    pub fn read_interface(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x08));
            (self.data.read() >> 8) as u8
        }
    }

    pub fn read_sub_class(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x08));
            (self.data.read() >> 16) as u8
        }
    }

    pub fn read_base_class(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x08));
            (self.data.read() >> 24) as u8
        }
    }

    pub fn read_cacheline_size(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x0c));
            self.data.read() as u8
        }
    }

    pub fn read_latency_timer(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x0c));
            (self.data.read() >> 8) as u8
        }
    }

    pub fn read_header_type(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x0c));
            (self.data.read() >> 16) as u8
        }
    }

    pub fn read_bist(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x0c));
            (self.data.read() >> 24) as u8
        }
    }

    pub fn read_base_address_register_0(&mut self, bus: u8, device: u8, function: u8) -> u32 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x10));
            self.data.read()
        }
    }

    pub fn read_base_address_register_1(&mut self, bus: u8, device: u8, function: u8) -> u32 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x14));
            self.data.read()
        }
    }

    pub fn read_base_address_register_2(&mut self, bus: u8, device: u8, function: u8) -> u32 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x18));
            self.data.read()
        }
    }

    pub fn read_base_address_register_3(&mut self, bus: u8, device: u8, function: u8) -> u32 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x1c));
            self.data.read()
        }
    }

    pub fn read_base_address_register_4(&mut self, bus: u8, device: u8, function: u8) -> u32 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x20));
            self.data.read()
        }
    }

    pub fn read_base_address_register_5(&mut self, bus: u8, device: u8, function: u8) -> u32 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x24));
            self.data.read()
        }
    }

    pub fn read_cardbus_cis_pointer(&mut self, bus: u8, device: u8, function: u8) -> u32 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x28));
            self.data.read()
        }
    }

    pub fn read_subsystem_vendor_id(&mut self, bus: u8, device: u8, function: u8) -> u16 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x2c));
            self.data.read() as u16
        }
    }

    pub fn read_subsystem_id(&mut self, bus: u8, device: u8, function: u8) -> u16 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x2c));
            (self.data.read() >> 16) as u16
        }
    }

    pub fn read_expansion_rom_base_address(&mut self, bus: u8, device: u8, function: u8) -> u32 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x030));
            self.data.read()
        }
    }

    pub fn read_capabilities_pointer(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x2c));
            self.data.read() as u8
        }
    }

    pub fn read_interruput_line(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x3c));
            self.data.read() as u8
        }
    }

    pub fn read_interruput_pin(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x3c));
            (self.data.read() >> 8) as u8
        }
    }

    pub fn read_min_gnt(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x3c));
            (self.data.read() >> 16) as u8
        }
    }

    pub fn read_max_lat(&mut self, bus: u8, device: u8, function: u8) -> u8 {
        unsafe {
            self.address
                .write(PCI::config_address(bus, device, function, 0x3c));
            (self.data.read() >> 24) as u8
        }
    }

    pub fn read_pci_configration(&mut self, bus: u8, device: u8, function: u8) -> PCIConfiguration {
        PCIConfiguration {
            vendor_id: self.read_vendor_id(bus, device, function),
            device_id: self.read_device_id(bus, device, function),
            command: self.read_command(bus, device, function),
            status: self.read_status(bus, device, function),
            revision_id: self.read_revision_id(bus, device, function),
            interface: self.read_interface(bus, device, function),
            sub_class: self.read_sub_class(bus, device, function),
            base_class: self.read_base_class(bus, device, function),
            chacheline_size: self.read_cacheline_size(bus, device, function),
            latency_timer: self.read_latency_timer(bus, device, function),
            header_type: self.read_header_type(bus, device, function),
            bist: self.read_bist(bus, device, function),
            base_address_register0: self.read_base_address_register_0(bus, device, function),
            base_address_register1: self.read_base_address_register_1(bus, device, function),
            base_address_register2: self.read_base_address_register_2(bus, device, function),
            base_address_register3: self.read_base_address_register_3(bus, device, function),
            base_address_register4: self.read_base_address_register_4(bus, device, function),
            base_address_register5: self.read_base_address_register_5(bus, device, function),
            cardbus_cis_pointer: self.read_cardbus_cis_pointer(bus, device, function),
            subsystem_vendor_id: self.read_subsystem_vendor_id(bus, device, function),
            subsystem_id: self.read_subsystem_id(bus, device, function),
            expansion_rom_base_address: self.read_expansion_rom_base_address(bus, device, function),
            capabilities_pointer: self.read_capabilities_pointer(bus, device, function),
            reserved0: None,
            reserved1: None,
            interrupt_line: self.read_interruput_line(bus, device, function),
            interrupt_pin: self.read_interruput_pin(bus, device, function),
            min_gnt: self.read_min_gnt(bus, device, function),
            max_lat: self.read_max_lat(bus, device, function),
        }
    }

    pub fn scan_all_bus(&mut self) -> Result<(), &'static str> {
        if self
            .read_pci_configration(0, 0, 0)
            .is_single_function_device()
        {
            self.scan_bus(0).unwrap();
        }

        for i in 1..8 {
            if self.read_vendor_id(0, 0, i) == 0xffff {
                continue;
            }
            self.scan_bus(i).unwrap();
        }

        Ok(())
    }

    pub fn scan_bus(&mut self, bus: u8) -> Result<(), &'static str> {
        for i in 0..32 {
            if self.read_vendor_id(bus, i, 0) == 0xffff {
                continue;
            }
            self.scan_device(bus, i).unwrap();
        }

        Ok(())
    }

    pub fn scan_device(&mut self, bus: u8, device: u8) -> Result<(), &'static str> {
        self.scan_function(bus, device, 0).unwrap();

        if self
            .read_pci_configration(bus, device, 0)
            .is_single_function_device()
        {
            return Ok(());
        }

        for i in 1..8 {
            if self.read_vendor_id(bus, device, i) == 0xffff {
                continue;
            }
            self.scan_function(bus, device, i).unwrap();
        }

        Ok(())
    }

    pub fn scan_function(&mut self, bus: u8, device: u8, function: u8) -> Result<(), &'static str> {
        let pci_configuration = self.read_pci_configration(bus, device, function);

        self.add_device(PCIDevice {
            bus: bus,
            device: device,
            function: function,
            pci_configuration: pci_configuration,
            device_type: pci_configuration.return_device_type(),
        })
        .unwrap();

        if pci_configuration.base_class == 0x06 && pci_configuration.sub_class == 0x04 {
            // PCI-to-PCI bridge
            let bus_num = pci_configuration.base_address_register2;
            let secondary_bus = bus_num >> 8;
            return self.scan_bus(secondary_bus as u8);
        }

        Ok(())
    }

    fn add_device(&mut self, pci_device: PCIDevice) -> Result<(), &'static str> {
        if self.device_index == self.devices.len() {
            return Err("Arrary is Full");
        }

        self.devices[self.device_index] = pci_device;
        self.device_index += 1;

        Ok(())
    }
}
