/// # Class Code Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.1.6 Class Code Register
#[repr(packed)]
pub struct Register {
    programming_interface: u8,
    sub_class: u8,
    base_class: u8,
}

/// # Class Code
/// ## References
/// * [PCI Code and ID Assignment Specification Revision 1.11](https://pcisig.com/sites/default/files/files/PCI_Code-ID_r_1_11__v24_Jan_2019.pdf)
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum Code {
    AllCurrentlyImplemented,                    // 00 00 00
    VgaCompatibleDevice,                        // 00 01 00
    Scsi {
        programming_interface: u8,
    },                                          // 01 00 xx
    Ide {
        programming_interface: u8,
    },                                          // 01 01 xx
    FloppyDisk,                                 // 01 02 00
    IpiBus,                                     // 01 03 00
    Raid,                                       // 01 04 00
    AtaSingleStepping,                          // 01 05 20
    AtaContinuousOperation,                     // 01 05 30
    SerialAtaVendorSpecific,                    // 01 06 00
    SerialAtaAhci,                              // 01 06 01
    SerialStorage,                              // 01 06 02
    SerialAttachedScsi,                         // 01 07 00
    Obsolete,                                   // 01 07 01
    NoneVolatileMemorySubsystemVendorSpecific,  // 01 08 00
    NoneVolatileMemorySubsystemNvmHci,          // 01 08 01
    NvmExpressIo,                               // 01 08 02
    NvmExpressAdministrative,                   // 01 08 03
    UniversalFlashStorageVendorSpecific,        // 01 09 00
    UniversalFlashStorageHostController,        // 01 09 01
    OtherMassStorage,                           // 01 80 00
    Ethernet,                                   // 02 00 00
    TokenRing,                                  // 02 01 00
    Fddi,                                       // 02 02 00
    Atm,                                        // 02 03 00
    Isdn,                                       // 02 04 00
    WorldFip,                                   // 02 05 00
    Picmg {
        programming_interface: u8,
    },                                          // 02 06 xx
    InfiniBand,                                 // 02 07 00
    HostFabric,                                 // 02 08 00
    OtherNetwork,                               // 02 80 00
    VgaCompatibleController,                    // 03 00 00
    Display8514Compatible,                      // 03 00 01
    Xga,                                        // 03 01 00
    Display3D,                                  // 03 02 00
    OtherDisplay,                               // 03 80 00
    Video,                                      // 04 00 00
    AudioDevice,                                // 04 01 00
    ComputerTelephony,                          // 04 02 00
    HighDefinitionAudio,                        // 04 03 00
    HighDefinitionAudioVendorSpecific,          // 04 03 80
    OtherMultimedia,                            // 04 80 00
    Ram,                                        // 05 00 00
    Flash,                                      // 05 01 00
    OtherMemory,                                // 05 80 00
    HostBridge,                                 // 06 00 00
    IsaBridge,                                  // 06 01 00
    EisaBridge,                                 // 06 02 00
    McaBridge,                                  // 06 03 00
    Pci2PciBridge,                              // 06 04 00
    SubtractiveDecodePci2PciBridge,             // 06 04 01
    PcmciaBridge,                               // 06 05 00
    NuBusBridge,                                // 06 06 00
    CardBusBridge,                              // 06 07 00
    RaceWayBridge {
        programming_interface: u8,
    },                                          // 06 08 xx
    SemiTransparentPci2PciBridgePrimary,        // 06 09 40
    SemiTransparentPci2PciBridgeSecondary,      // 06 09 80
    InfiniBand2PciHostBridge,                   // 06 0a 00
    AdvancedSwitching2PciHostBridgeCustom,      // 06 0b 00
    AdvancedSwitching2PciHostBridgeASISIG,      // 06 0b 01
    OtherBridge,                                // 06 80 00
    SerialGenericXTCompatible,                  // 07 00 00
    Serial16450Compatible,                      // 07 00 01
    Serial16550Compatible,                      // 07 00 02
    Serial16650Compatible,                      // 07 00 03
    Serial16750Compatible,                      // 07 00 04
    Serial16850Compatible,                      // 07 00 05
    Serial16950Compatible,                      // 07 00 06
    ParallelPort,                               // 07 01 00
    BidirectionalParallelPort,                  // 07 01 01
    Ecp1xCompliantParallelPort,                 // 07 01 02
    Ieee1284Controller,                         // 07 01 03
    Ieee1284TargetDevice,                       // 07 01 fe
    MultiportSerial,                            // 07 02 00
    GenericModem,                               // 07 03 00
    HayesCompatibleModem16450Compatible,        // 07 03 01
    HayesCompatibleModem16550Compatible,        // 07 03 02
    HayesCompatibleModem16650Compatible,        // 07 03 03
    HayesCompatibleModem16750Compatible,        // 07 03 04
    Gpib,                                       // 07 04 00
    SmartCard,                                  // 07 05 00
    OtherCommunication,                         // 07 80 00
    Generic8259Pic,                             // 08 00 00
    IsaPic,                                     // 08 00 01
    EisaPic,                                    // 08 00 02
    IoApicInterrupt,                            // 08 00 10
    IoxApicInterrupt,                           // 08 00 20
    Generic8237Dma,                             // 08 01 00
    IsaDma,                                     // 08 01 01
    EisaDma,                                    // 08 01 02
    Generic8254SystemTimer,                     // 08 02 00
    IsaSystemTimer,                             // 08 02 01
    EisaSystemTimer,                            // 08 02 02
    HighPerformanceEventTimer,                  // 08 02 03
    GenericRtc,                                 // 08 03 00
    IsaRtc,                                     // 08 03 01
    GenericPciHotPlug,                          // 08 04 00
    SdHost,                                     // 08 05 00
    Iommu,                                      // 08 06 00
    RootComplexEvent,                           // 08 07 00
    OtherSystemPeripheral,                      // 08 80 00
    Kerboard,                                   // 09 00 00
    Digitizer,                                  // 09 01 00
    Mouse,                                      // 09 02 00
    Scanner,                                    // 09 03 00
    GenericGameport,                            // 09 04 00
    Gameport,                                   // 09 04 10
    OtherInput,                                 // 09 80 00
    GenericDockingStation,                      // 0a 00 00
    OtherDockingStation,                        // 0a 80 00
    Processor386,                               // 0b 00 00
    Processor486,                               // 0b 01 00
    Pentium,                                    // 0b 02 00
    Alpha,                                      // 0b 10 00
    PowerPC,                                    // 0b 20 00
    Mips,                                       // 0b 30 00
    Coprocessor,                                // 0b 40 00
    OtherProcessor,                             // 0b 80 00
    Ieee1394,                                   // 0c 00 00
    Ieee1394OpenHci,                            // 0c 00 10
    AccessBus,                                  // 0c 01 00
    Ssa,                                        // 0c 02 00
    UsbUhci,                                    // 0c 03 00
    UsbOhci,                                    // 0c 03 10
    UsbEhci,                                    // 0c 03 20
    UsbXhci,                                    // 0c 03 30
    UsbNoSpecificProgrammingInterface,          // 0c 03 80
    UsbNoHostController,                        // 0c 03 fe
    FibreChannel,                               // 0c 04 00
    SmBus,                                      // 0c 05 00
    InfiniBandDeprecated,                       // 0c 06 00
    IpmiSmic,                                   // 0c 07 00
    IpmiKeyboardControllerStyle,                // 0c 07 01
    IpmiBlockTransfer,                          // 0c 07 02
    Sercos,                                     // 0c 08 00
    CanBus,                                     // 0c 09 00
    MipiI3c,                                    // 0c 0a 00
    OtherSerialBus,                             // 0c 80 00
    Irda,                                       // 0d 00 00
    ConsumerIR,                                 // 0d 01 00
    UwbRadio,                                   // 0d 01 10
    Rf,                                         // 0d 10 00
    Bluetooth,                                  // 0d 11 00
    Broadband,                                  // 0d 12 00
    Ethernet80211a,                             // 0d 20 00
    Ethernet20811b,                             // 0d 21 00
    Cellular,                                   // 0d 40 00
    CellularPlusEthernet,                       // 0d 41 00
    OtherWireless,                              // 0d 80 00
    IntelligentIo {
        programming_interface: u8,
    },                                          // 0e 00 xx
    MessageFifo,                                // 0e 00 00
    Tv,                                         // 0f 01 00
    Audio,                                      // 0f 02 00
    Voice,                                      // 0f 03 00
    Data,                                       // 0f 04 00
    OtherSatelliteCommunication,                // 0f 80 00
    NetworkAndComputingEncryptionAndDecryption, // 10 00 00
    EntertainmentEncryptionAndDecryption,       // 10 10 00
    OtherEncryptionAndDecryption,               // 10 80 00
    Dpio,                                       // 11 00 00
    PerformanceCounter,                         // 11 01 00
    CommunicationSynchronizationPlusTime,       // 11 10 00
    ManagementCard,                             // 11 20 00
    OtherDataAcquisitionAndSignalProcessing,    // 11 80 00
    ProcessingAccelerator,                      // 12 00 00
    NonEssentialInstrumentationFunction,        // 13 00 00
    Other {
        base_class: u8,
        sub_class: u8,
        programming_interface: u8,
    },
}

impl From<Register> for Code {
    fn from(register: Register) -> Self {
        let Register {
            programming_interface,
            sub_class,
            base_class,
        } = register;
        match (base_class, sub_class, programming_interface) {
            (0x00, 0x00, 0x00) => Self::AllCurrentlyImplemented,                    // 00 00 00
            (0x00, 0x01, 0x00) => Self::VgaCompatibleDevice,                        // 00 01 00
            (0x01, 0x00, programming_interface) => Self::Scsi {
                programming_interface,
            },                                                                      // 01 00 xx
            (0x01, 0x01, programming_interface) => Self::Ide {
                programming_interface,
            },                                                                      // 01 01 xx
            (0x01, 0x02, 0x00) => Self::FloppyDisk,                                 // 01 02 00
            (0x01, 0x03, 0x00) => Self::IpiBus,                                     // 01 03 00
            (0x01, 0x04, 0x00) => Self::Raid,                                       // 01 04 00
            (0x01, 0x05, 0x20) => Self::AtaSingleStepping,                          // 01 05 20
            (0x01, 0x05, 0x30) => Self::AtaContinuousOperation,                     // 01 05 30
            (0x01, 0x06, 0x00) => Self::SerialAtaVendorSpecific,                    // 01 06 00
            (0x01, 0x06, 0x01) => Self::SerialAtaAhci,                              // 01 06 01
            (0x01, 0x06, 0x02) => Self::SerialStorage,                              // 01 06 02
            (0x01, 0x07, 0x00) => Self::SerialAttachedScsi,                         // 01 07 00
            (0x01, 0x07, 0x01) => Self::Obsolete,                                   // 01 07 01
            (0x01, 0x08, 0x00) => Self::NoneVolatileMemorySubsystemVendorSpecific,  // 01 08 00
            (0x01, 0x08, 0x01) => Self::NoneVolatileMemorySubsystemNvmHci,          // 01 08 01
            (0x01, 0x08, 0x02) => Self::NvmExpressIo,                               // 01 08 02
            (0x01, 0x08, 0x03) => Self::NvmExpressAdministrative,                   // 01 08 03
            (0x01, 0x09, 0x00) => Self::UniversalFlashStorageVendorSpecific,        // 01 09 00
            (0x01, 0x09, 0x01) => Self::UniversalFlashStorageHostController,        // 01 09 01
            (0x01, 0x80, 0x00) => Self::OtherMassStorage,                           // 01 80 00
            (0x02, 0x00, 0x00) => Self::Ethernet,                                   // 02 00 00
            (0x02, 0x01, 0x00) => Self::TokenRing,                                  // 02 01 00
            (0x02, 0x02, 0x00) => Self::Fddi,                                       // 02 02 00
            (0x02, 0x03, 0x00) => Self::Atm,                                        // 02 03 00
            (0x02, 0x04, 0x00) => Self::Isdn,                                       // 02 04 00
            (0x02, 0x05, 0x00) => Self::WorldFip,                                   // 02 05 00
            (0x02, 0x06, programming_interface) => Self::Picmg {
                programming_interface
            },                                                                      // 02 06 xx
            (0x02, 0x07, 0x00) => Self::InfiniBand,                                 // 02 07 00
            (0x02, 0x08, 0x00) => Self::HostFabric,                                 // 02 08 00
            (0x02, 0x80, 0x00) => Self::OtherNetwork,                               // 02 80 00
            (0x03, 0x00, 0x00) => Self::VgaCompatibleController,                    // 03 00 00
            (0x03, 0x00, 0x01) => Self::Display8514Compatible,                      // 03 00 01
            (0x03, 0x01, 0x00) => Self::Xga,                                        // 03 01 00
            (0x03, 0x02, 0x00) => Self::Display3D,                                  // 03 02 00
            (0x03, 0x80, 0x00) => Self::OtherDisplay,                               // 03 80 00
            (0x04, 0x00, 0x00) => Self::Video,                                      // 04 00 00
            (0x04, 0x01, 0x00) => Self::AudioDevice,                                // 04 01 00
            (0x04, 0x02, 0x00) => Self::ComputerTelephony,                          // 04 02 00
            (0x04, 0x03, 0x00) => Self::HighDefinitionAudio,                        // 04 03 00
            (0x04, 0x03, 0x80) => Self::HighDefinitionAudioVendorSpecific,          // 04 03 80
            (0x04, 0x80, 0x00) => Self::OtherMultimedia,                            // 04 80 00
            (0x05, 0x00, 0x00) => Self::Ram,                                        // 05 00 00
            (0x05, 0x01, 0x00) => Self::Flash,                                      // 05 01 00
            (0x05, 0x80, 0x00) => Self::OtherMemory,                                // 05 80 00
            (0x06, 0x00, 0x00) => Self::HostBridge,                                 // 06 00 00
            (0x06, 0x01, 0x00) => Self::IsaBridge,                                  // 06 01 00
            (0x06, 0x02, 0x00) => Self::EisaBridge,                                 // 06 02 00
            (0x06, 0x03, 0x00) => Self::McaBridge,                                  // 06 03 00
            (0x06, 0x04, 0x00) => Self::Pci2PciBridge,                              // 06 04 00
            (0x06, 0x04, 0x01) => Self::SubtractiveDecodePci2PciBridge,             // 06 04 01
            (0x06, 0x05, 0x00) => Self::PcmciaBridge,                               // 06 05 00
            (0x06, 0x06, 0x00) => Self::NuBusBridge,                                // 06 06 00
            (0x06, 0x07, 0x00) => Self::CardBusBridge,                              // 06 07 00
            (0x06, 0x08, programming_interface) => Self::RaceWayBridge {
                programming_interface,
            },                                                                      // 06 08 xx
            (0x06, 0x09, 0x40) => Self::SemiTransparentPci2PciBridgePrimary,        // 06 09 40
            (0x06, 0x09, 0x80) => Self::SemiTransparentPci2PciBridgeSecondary,      // 06 09 80
            (0x06, 0x0a, 0x00) => Self::InfiniBand2PciHostBridge,                   // 06 0a 00
            (0x06, 0x0b, 0x00) => Self::AdvancedSwitching2PciHostBridgeCustom,      // 06 0b 00
            (0x06, 0x0b, 0x01) => Self::AdvancedSwitching2PciHostBridgeASISIG,      // 06 0b 01
            (0x06, 0x80, 0x00) => Self::OtherBridge,                                // 06 80 00
            (0x07, 0x00, 0x00) => Self::SerialGenericXTCompatible,                  // 07 00 00
            (0x07, 0x00, 0x01) => Self::Serial16450Compatible,                      // 07 00 01
            (0x07, 0x00, 0x02) => Self::Serial16550Compatible,                      // 07 00 02
            (0x07, 0x00, 0x03) => Self::Serial16650Compatible,                      // 07 00 03
            (0x07, 0x00, 0x04) => Self::Serial16750Compatible,                      // 07 00 04
            (0x07, 0x00, 0x05) => Self::Serial16850Compatible,                      // 07 00 05
            (0x07, 0x00, 0x06) => Self::Serial16950Compatible,                      // 07 00 06
            (0x07, 0x01, 0x00) => Self::ParallelPort,                               // 07 01 00
            (0x07, 0x01, 0x01) => Self::BidirectionalParallelPort,                  // 07 01 01
            (0x07, 0x01, 0x02) => Self::Ecp1xCompliantParallelPort,                 // 07 01 02
            (0x07, 0x01, 0x03) => Self::Ieee1284Controller,                         // 07 01 03
            (0x07, 0x01, 0xfe) => Self::Ieee1284TargetDevice,                       // 07 01 fe
            (0x07, 0x02, 0x00) => Self::MultiportSerial,                            // 07 02 00
            (0x07, 0x03, 0x00) => Self::GenericModem,                               // 07 03 00
            (0x07, 0x03, 0x01) => Self::HayesCompatibleModem16450Compatible,        // 07 03 01
            (0x07, 0x03, 0x02) => Self::HayesCompatibleModem16550Compatible,        // 07 03 01
            (0x07, 0x03, 0x03) => Self::HayesCompatibleModem16650Compatible,        // 07 03 01
            (0x07, 0x03, 0x04) => Self::HayesCompatibleModem16750Compatible,        // 07 03 01
            (0x07, 0x04, 0x00) => Self::Gpib,                                       // 07 04 00
            (0x07, 0x05, 0x00) => Self::SmartCard,                                  // 07 05 00
            (0x07, 0x80, 0x00) => Self::OtherCommunication,                         // 07 80 00
            (0x08, 0x00, 0x00) => Self::Generic8259Pic,                             // 08 00 00
            (0x08, 0x00, 0x01) => Self::IsaPic,                                     // 08 00 01
            (0x08, 0x00, 0x02) => Self::EisaPic,                                    // 08 00 02
            (0x08, 0x00, 0x10) => Self::IoApicInterrupt,                            // 08 00 10
            (0x08, 0x00, 0x20) => Self::IoxApicInterrupt,                           // 08 00 20
            (0x08, 0x01, 0x00) => Self::Generic8237Dma,                             // 08 01 00
            (0x08, 0x01, 0x01) => Self::IsaDma,                                     // 08 01 01
            (0x08, 0x01, 0x02) => Self::EisaDma,                                    // 08 01 02
            (0x08, 0x02, 0x00) => Self::Generic8254SystemTimer,                     // 08 02 00
            (0x08, 0x02, 0x01) => Self::IsaSystemTimer,                             // 08 02 01
            (0x08, 0x02, 0x02) => Self::EisaSystemTimer,                            // 08 02 02
            (0x08, 0x02, 0x03) => Self::HighPerformanceEventTimer,                  // 08 02 03
            (0x08, 0x03, 0x00) => Self::GenericRtc,                                 // 08 03 00
            (0x08, 0x03, 0x01) => Self::IsaRtc,                                     // 08 03 01
            (0x08, 0x04, 0x00) => Self::GenericPciHotPlug,                          // 08 04 00
            (0x08, 0x05, 0x00) => Self::SdHost,                                     // 08 05 00
            (0x08, 0x06, 0x00) => Self::Iommu,                                      // 08 06 00
            (0x08, 0x07, 0x00) => Self::RootComplexEvent,                           // 08 07 00
            (0x08, 0x80, 0x00) => Self::OtherSystemPeripheral,                      // 08 80 00
            (0x09, 0x00, 0x00) => Self::Kerboard,                                   // 09 00 00
            (0x09, 0x01, 0x00) => Self::Digitizer,                                  // 09 01 00
            (0x09, 0x02, 0x00) => Self::Mouse,                                      // 09 02 00
            (0x09, 0x03, 0x00) => Self::Scanner,                                    // 09 03 00
            (0x09, 0x04, 0x00) => Self::GenericGameport,                            // 09 04 00
            (0x09, 0x04, 0x10) => Self::Gameport,                                   // 09 04 10
            (0x09, 0x80, 0x00) => Self::OtherInput,                                 // 09 80 00
            (0x0a, 0x00, 0x00) => Self::GenericDockingStation,                      // 0a 00 00
            (0x0a, 0x80, 0x00) => Self::OtherDockingStation,                        // 0a 80 00
            (0x0b, 0x00, 0x00) => Self::Processor386,                               // 0b 00 00
            (0x0b, 0x01, 0x00) => Self::Processor486,                               // 0b 01 00
            (0x0b, 0x02, 0x00) => Self::Pentium,                                    // 0b 02 00
            (0x0b, 0x10, 0x00) => Self::Alpha,                                      // 0b 10 00
            (0x0b, 0x20, 0x00) => Self::PowerPC,                                    // 0b 20 00
            (0x0b, 0x30, 0x00) => Self::Mips,                                       // 0b 30 00
            (0x0b, 0x40, 0x00) => Self::Coprocessor,                                // 0b 40 00
            (0x0b, 0x80, 0x00) => Self::OtherProcessor,                             // 0b 80 00
            (0x0c, 0x00, 0x00) => Self::Ieee1394,                                   // 0c 00 00
            (0x0c, 0x00, 0x10) => Self::Ieee1394OpenHci,                            // 0c 00 10
            (0x0c, 0x01, 0x00) => Self::AccessBus,                                  // 0c 01 00
            (0x0c, 0x02, 0x00) => Self::Ssa,                                        // 0c 02 00
            (0x0c, 0x03, 0x00) => Self::UsbUhci,                                    // 0c 03 00
            (0x0c, 0x03, 0x10) => Self::UsbOhci,                                    // 0c 03 10
            (0x0c, 0x03, 0x20) => Self::UsbEhci,                                    // 0c 03 20
            (0x0c, 0x03, 0x30) => Self::UsbXhci,                                    // 0c 03 30
            (0x0c, 0x03, 0x80) => Self::UsbNoSpecificProgrammingInterface,          // 0c 03 80
            (0x0c, 0x03, 0xfe) => Self::UsbNoHostController,                        // 0c 03 fe
            (0x0c, 0x04, 0x00) => Self::FibreChannel,                               // 0c 04 00
            (0x0c, 0x05, 0x00) => Self::SmBus,                                      // 0c 05 00
            (0x0c, 0x06, 0x00) => Self::InfiniBandDeprecated,                       // 0c 06 00
            (0x0c, 0x07, 0x00) => Self::IpmiSmic,                                   // 0c 07 00
            (0x0c, 0x07, 0x01) => Self::IpmiKeyboardControllerStyle,                // 0c 07 01
            (0x0c, 0x07, 0x02) => Self::IpmiBlockTransfer,                          // 0c 07 02
            (0x0c, 0x08, 0x00) => Self::Sercos,                                     // 0c 08 00
            (0x0c, 0x09, 0x00) => Self::CanBus,                                     // 0c 09 00
            (0x0c, 0x0a, 0x00) => Self::MipiI3c,                                    // 0c 0a 00
            (0x0c, 0x80, 0x00) => Self::OtherSerialBus,                             // 0c 80 00
            (0x0d, 0x00, 0x00) => Self::Irda,                                       // 0d 00 00
            (0x0d, 0x01, 0x00) => Self::ConsumerIR,                                 // 0d 01 00
            (0x0d, 0x01, 0x10) => Self::UwbRadio,                                   // 0d 01 10
            (0x0d, 0x10, 0x00) => Self::Rf,                                         // 0d 10 00
            (0x0d, 0x11, 0x00) => Self::Bluetooth,                                  // 0d 11 00
            (0x0d, 0x12, 0x00) => Self::Broadband,                                  // 0d 12 00
            (0x0d, 0x20, 0x00) => Self::Ethernet80211a,                             // 0d 20 00
            (0x0d, 0x21, 0x00) => Self::Ethernet20811b,                             // 0d 21 00
            (0x0d, 0x40, 0x00) => Self::Cellular,                                   // 0d 40 00
            (0x0d, 0x41, 0x00) => Self::CellularPlusEthernet,                       // 0d 41 00
            (0x0d, 0x80, 0x00) => Self::OtherWireless,                              // 0d 80 00
            (0x0e, 0x00, 0x00) => Self::MessageFifo,                                // 0e 00 00
            (0x0e, 0x00, programming_interface) => Self::IntelligentIo {
                programming_interface,
            },                                                                      // 0e 00 xx
            (0x0f, 0x01, 0x00) => Self::Tv,                                         // 0f 01 00
            (0x0f, 0x02, 0x00) => Self::Audio,                                      // 0f 02 00
            (0x0f, 0x03, 0x00) => Self::Voice,                                      // 0f 03 00
            (0x0f, 0x04, 0x00) => Self::Data,                                       // 0f 04 00
            (0x0f, 0x80, 0x00) => Self::OtherSatelliteCommunication,                // 0f 80 00
            (0x10, 0x00, 0x00) => Self::NetworkAndComputingEncryptionAndDecryption, // 10 00 00
            (0x10, 0x10, 0x00) => Self::EntertainmentEncryptionAndDecryption,       // 10 10 00
            (0x10, 0x80, 0x00) => Self::OtherEncryptionAndDecryption,               // 10 80 00
            (0x11, 0x00, 0x00) => Self::Dpio,                                       // 11 00 00
            (0x11, 0x01, 0x00) => Self::PerformanceCounter,                         // 11 01 00
            (0x11, 0x10, 0x00) => Self::CommunicationSynchronizationPlusTime,       // 11 10 00
            (0x11, 0x20, 0x00) => Self::ManagementCard,                             // 11 20 00
            (0x11, 0x80, 0x00) => Self::OtherDataAcquisitionAndSignalProcessing,    // 11 80 00
            (0x12, 0x00, 0x00) => Self::ProcessingAccelerator,                      // 12 00 00
            (0x13, 0x00, 0x00) => Self::NonEssentialInstrumentationFunction,        // 13 00 00
            (base_class, sub_class, programming_interface) => Self::Other {
                base_class,
                sub_class,
                programming_interface,
            },
        }
    }
}

