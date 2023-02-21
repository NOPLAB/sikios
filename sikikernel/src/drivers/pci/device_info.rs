#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
#[repr(u8)]
pub enum PCIClassCode {
    Unclassified(Unclassified),
    MassStorageController(MassStorageController),
    NetworkController(NetworkController),
    DisplayController(DisplayController),
    MultimediaController(MultimediaController),
    MemoryController(MemoryController),
    Bridge(Bridge),
    SimpleCommunicationController(SimpleCommunicationController),
    BaseSystemPeripheral(BaseSystemPeripheral),
    InputDeviceController(InputDeviceController),
    DockingStation(DockingStation),
    Processor(PCISubClassProcessor),
    SerialBusController(SerialBusController),
    WirelessController(PCISubClassWirelessController),
    IntelligentController(PCISubClassIntelligentController),
    SatelliteCommunicationController(SatelliteCommunicationController),
    EncryptionController(EncryptionController),
    SignalProcessingController(SignalProcessingController),
    ProcessingAccelerator,
    NonEssentialInstrumentation,
    Reserved0x3F,
    CoProcessor,
    Reserved0xFE,
    #[default]
    UnassignedClass,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Unclassified {
    NonVGACompatibleUnclassifiedDevice = 0x0,
    VGACompatibleUnclassifiedDevice = 0x1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MassStorageController {
    SCSIBusController = 0x0,
    IDEController = 0x1,
    FloppyDiskController = 0x2,
    IPIBusController = 0x3,
    RAIDController = 0x4,
    ATAController = 0x5,
    SerialATAController = 0x6,
    SerialAttachedSCSIController = 0x7,
    NonVolatileMemoryController = 0x8,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NetworkController {
    EthernetController = 0x0,
    TokenRingController = 0x1,
    FDDIController = 0x2,
    ATMController = 0x3,
    ISDNController = 0x4,
    WorldFipController = 0x5,
    PICMG214MultiComputingController = 0x6,
    InfinibandController = 0x7,
    FabricController = 0x8,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DisplayController {
    VGACompatibleController = 0x0,
    XGAController = 0x1,
    _3DController = 0x2,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MultimediaController {
    MultimediaVideoController = 0x0,
    MultimediaAudioController = 0x1,
    ComputerTelephonyDevice = 0x2,
    AudioDevice = 0x3,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MemoryController {
    RAMController = 0x0,
    FlashController = 0x1,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bridge {
    HostBridge = 0x0,
    ISABridge = 0x1,
    EISABridge = 0x2,
    MCABridge = 0x3,
    PCItoPCIBridge1 = 0x4,
    PCMCIABridge = 0x5,
    NuBusBridge = 0x6,
    CardBusBridge = 0x7,
    RACEwayBridge = 0x8,
    PCItoPCIBridge2 = 0x9,
    InfiniBandtoPCIHostBridge = 0xa,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SimpleCommunicationController {
    SerialController = 0x0,
    ParallelController = 0x1,
    MultiportSerialController = 0x2,
    Modem = 0x3,
    IEEE48812Controller = 0x4,
    SmartCardController = 0x5,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BaseSystemPeripheral {
    PIC = 0x0,
    DMAConrtoller = 0x1,
    Timer = 0x2,
    RTCController = 0x3,
    PCIHotPlugController = 0x4,
    SDHostController = 0x5,
    IOMMU = 0x6,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InputDeviceController {
    KeyboardController = 0x0,
    DigitizerPen = 0x1,
    MouseController = 0x2,
    ScannerController = 0x3,
    GameportController = 0x4,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DockingStation {
    Generic = 0x0,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PCISubClassProcessor {
    _386 = 0x0,
    _486 = 0x1,
    Pentium = 0x2,
    PentiumPro = 0x3,
    Alpha = 0x10,
    PowerPC = 0x20,
    MIPS = 0x30,
    CoProcessor = 0x40,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum SerialBusController {
    FireWireController = 0x0,
    ACCESSBusController = 0x1,
    SSA = 0x2,
    USBController(USBController),
    FibreChannel = 0x4,
    SMBusController = 0x5,
    InfiniBandController = 0x6,
    IPMIInterface = 0x7,
    SERCOSInterface = 0x8,
    CANbusController = 0x9,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PCISubClassWirelessController {
    iRDACompatibleController = 0x0,
    ConsumerIRController = 0x1,
    RFController = 0x10,
    BluetoothController = 0x11,
    BroadbandController = 0x12,
    EthernetController8021a = 0x20,
    EthernetController8021b = 0x21,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PCISubClassIntelligentController {
    I20 = 0x0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SatelliteCommunicationController {
    SatelliteTVController = 0x1,
    SatelliteAudioController = 0x2,
    SatelliteVoiceController = 0x3,
    SatelliteDataController = 0x4,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EncryptionController {
    NetworkAndComputingEncryptionDecryption = 0x0,
    EntertainmentEncryptionDecryption = 0x10,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SignalProcessingController {
    DPIOModules = 0x0,
    PerformanceCounters = 0x1,
    CommunicationSynchronizer = 0x10,
    SignalProcessingManagement = 0x20,
    Other = 0x80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum USBController {
    UHCIController,
    OHCIController,
    EHCIController,
    XHCIController,
    Unspecified,
    USBDevice,
}
