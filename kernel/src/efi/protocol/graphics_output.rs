//! # Graphics Output Protocol
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.9 Graphics Output Protocol

use super::super::{
    Guid,
    Status,
    SystemTable,
    Void,
    memory,
    null,
};

/// # EFI_GRAPHICS_OUTPUT_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.9.1 Blt Buffer
#[derive(Debug)]
#[repr(C)]
pub struct Protocol<'a> {
    query_mode: QueryMode,
    set_mode: SetMode,
    blt: Blt,
    mode: &'a Mode<'a>,
}

impl Protocol<'static> {
    #[allow(dead_code)]
    pub fn get() -> &'static Self {
        let guid = Guid::new(0x9042a9de, 0x23dc, 0x4a38, [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a]);
        let registration: &Void = null();
        let protocol: &Void = SystemTable::get()
            .locate_protocol(registration, guid)
            .unwrap();
        let protocol: *const Void = protocol as *const Void;
        let protocol: *const Protocol = protocol as *const Protocol;
        unsafe {
            &*protocol
        }
    }
}

/// # EFI_PIXEL_BITMASK
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.9.1 Blt Buffer
#[derive(Debug)]
#[repr(C)]
pub struct PixelBitmask {
    red: u32,
    green: u32,
    blue: u32,
    reserved: u32,
}

/// # EFI_GRAPHICS_PIXEL_FORMAT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.9.1 Blt Buffer
#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub enum PixelFormat {
    RedGreenBlue,
    BlueGreenRed,
    BitMask,
    BltOnly,
    FormatMax,
}

/// # EFI_GRAPHICS_OUTPUT_MODE_INFORMATION
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.9.1 Blt Buffer
#[derive(Debug)]
#[repr(C)]
pub struct ModeInformation {
    version: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: PixelFormat,
    pixel_information: PixelBitmask,
    pixels_per_scan_line: u32,
}

/// # EVI_GRAPHICS_OUTPUT_PROTOCOL_MODE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.9.1 Blt Buffer
#[derive(Debug)]
#[repr(C)]
pub struct Mode<'a> {
    max_mode: u32,
    mode: u32,
    info: &'a ModeInformation,
    size_of_info: usize,
    frame_buffer_base: memory::PhysicalAddress,
    frame_buffer_size: usize,
}

/// # EFI_GRAPHICS_OUTPUT_PROTOCOL_QUERY_MODE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.9.1 Blt Buffer
type QueryMode = extern "efiapi" fn(/* This */ &Protocol, /* ModeNumber */ u32, /* SizeOfInfo */ &mut usize, /* Info */ &mut &ModeInformation) -> Status;

/// # EFI_GRAPHICS_OUTPUT_PROTOCOL_SET_MODE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.9.1 Blt Buffer
type SetMode = extern "efiapi" fn(/* This */ &Protocol, /* ModeNumber */ u32) -> Status;

/// # EFI_GRAPHICS_OUTPUT_BLT_PIXEL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.9.1 Blt Buffer
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct BltPixel {
    blue: u8,
    green: u8,
    red: u8,
    reserved: u8,
}

/// # EFI_GRAPHICS_OUTPUT_BLT_OPERATION
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.9.1 Blt Buffer
#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub enum BltOperation {
    VideoFill,
    VideoToBltBuffer,
    BufferToVideo,
    VideoToVideo,
    Max,
}

/// # EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.9.1 Blt Buffer
type Blt = extern "efiapi" fn(/* This */ &Protocol, /* BltBuffer */ &mut BltPixel, /* BltOperation */ BltOperation, /* SourceX */ usize, /* SourceY */ usize, /* DestinationX */ usize, /* DestinationY */ usize, /* Width */ usize, /* Height */ usize, /* Delta */ usize) -> Status;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    #[allow(dead_code)]
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }

    #[allow(dead_code)]
    pub fn x(&self) -> usize {
        self.x
    }

    #[allow(dead_code)]
    pub fn y(&self) -> usize {
        self.y
    }
}

