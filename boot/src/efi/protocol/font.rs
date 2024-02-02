//! # Font Protocol
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.1 Font Protocol

use {
    alloc::{
        collections::BTreeMap,
        string::String,
    },
    bitfield_struct::bitfield,
    core::{
        fmt,
        slice,
    },
    super::super::{
        Char16,
        Char8,
        Guid,
        Status,
        SystemTable,
        Void,
        char16,
        graphics_output,
        hii,
        null,
    },
};

/// # EFI_HII_FONT_STYLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 33.3.3.1 Fixed Header
#[bitfield(u32)]
pub struct Style {
    bold: bool,
    italic: bool,
    #[bits(14, access = RO)]
    reserved0: u16,
    emboss: bool,
    outline: bool,
    shadow: bool,
    underline: bool,
    dbl_under: bool,
    #[bits(11, access = RO)]
    reserved1: u16,
}

/// # EFI_STRING_ID
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 33.3.8.2.1 EFI_IFR_OP_HEADER
pub type StringId = u16;

/// # EFI_HII_FONT_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.1 Font Protocol
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    string_to_image: StringToImage,
    string_id_to_image: StringIdToImage,
    get_glyph: GetGlyph,
    get_font_info: GetFontInfo,
}

impl Protocol {
    pub fn get() -> &'static Self {
        let guid = Guid::new(0xe9ca4775, 0x8657, 0x47fc, [0x97, 0xe7, 0x7e, 0xd6, 0x5a, 0x8, 0x43, 0x24]);
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

    pub fn fonts(&self) -> BTreeMap<usize, Font> {
        let font_iterator: FontIterator = self.into();
        font_iterator
            .enumerate()
            .map(|(font_number, display_info)| (font_number, Font::new(display_info, ('!'..='~')
                .filter_map(|character| {
                    let mut blt: &ImageOutput = null();
                    let mut base_line: usize = 0;
                    (self.get_glyph)(self, character as Char16, display_info, &mut blt, &mut base_line)
                        .result()
                        .ok()
                        .map(|_| (character, (0..blt.width)
                            .flat_map(|x| (0..blt.height)
                                .map(move |y| graphics_output::Coordinates::new(x as usize, y as usize)))
                            .map(|coordinates| (coordinates, blt.pixel(coordinates.x(), coordinates.y()) == display_info.foreground_color))
                            .collect()))
                })
                .collect())
            ))
            .collect()
    }
}

/// # EFI_HII_STRING_TO_IMAGE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.1 Font Protocol
type StringToImage = extern "efiapi" fn(/* This */ &Protocol, /* Flags */ OutFlags, /* String */ EfiString<'_>, /* StringInfo */ &DisplayInfo<'_>, /* Blt */ &mut &ImageOutput<'_>, /* BltX */ usize, /* BltY */ usize, /* RowInfoArray */ &mut &RowInfo, /* RowInfoArraySize */ &mut usize, /* ColumnInfoArray */ &mut usize) -> Status;

/// # EFI_HII_OUT_FLAGS
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.1 Font Protocol
#[bitfield(u32)]
pub struct OutFlags {
    clip: bool,
    wrap: bool,
    clip_clean_y: bool,
    clip_clean_x: bool,
    transparent: bool,
    ignore_if_no_glyph: bool,
    ignore_line_break: bool,
    direct_to_screen: bool,
    #[bits(24)]
    reserved: u32,
}

/// # EFI_STRING
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.1 Font Protocol
pub type EfiString<'a> = char16::NullTerminatedString<'a>;

/// # EFI_HII_ROW_INFO
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.1 Font Protocol
#[derive(Debug)]
#[repr(C)]
pub struct RowInfo {
    start_index: usize,
    end_index: usize,
    line_height: usize,
    line_width: usize,
    base_line_offset: usize,
}

/// # EFI_HII_STRING_ID_TO_IMAGE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.1 Font Protocol
type StringIdToImage = extern "efiapi" fn(/* This */ &Protocol, /* Flags */ OutFlags, /* PackageList */ hii::Handle, /* StringId */ StringId, /* Language */ &Char8, /* StringInfo */ &DisplayInfo<'_>, /* Blt */ &mut &ImageOutput<'_>, /* BltX */ usize, /* BltY */ usize, /* RowInfoArray */ &mut &RowInfo, /* RowInfoArraySize*/ &mut usize, /* ColumnInfoArray */ &mut usize) -> Status;

/// # EFI_HII_GET_GLYPH
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.1 Font Protocol
type GetGlyph = extern "efiapi" fn(/* This */ &Protocol, /* Char */ Char16, /* StringInfo */ &DisplayInfo<'_>, /* Blt */ &mut &ImageOutput<'_>, /* BaseLine */ &mut usize) -> Status;

/// # EFI_HII_GET_FONT_INFO
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.1 Font Protocol
type GetFontInfo = extern "efiapi" fn(/* This */ &Protocol, /* FontHandle */ &mut Handle<'_>, /* StringInfoIn */ &DisplayInfo, /* StringInfoOut */ &mut &DisplayInfo, /* String */ EfiString<'_>) -> Status;

/// # EFI_FONT_HANDLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.1 Font Protocol
pub type Handle<'a> = &'a Void;

/// # EFI_FONT_DISPLAY_INFO
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.2.1 Code Definitions
#[repr(C)]
pub struct DisplayInfo<'a> {
    foreground_color: graphics_output::BltPixel,
    background_color: graphics_output::BltPixel,
    info_mask: InfoMask,
    info: Info<'a>,
}

impl DisplayInfo<'_> {
    fn name(&self) -> Option<String> {
        (!self.info_mask.sys_font()).then(|| {
            (&self.info.name)
                .into()
        })
    }
}

impl fmt::Debug for DisplayInfo<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DisplayInfo")
            .field("foreground_color", &self.foreground_color)
            .field("background_color", &self.background_color)
            .field("info_mask", &self.info_mask)
            .field("info.style", &self.info.style)
            .field("info.size", &self.info.size)
            .field("info.name", &self.name())
            .finish()
    }
}

/// # EFI_FONT_INFO_MASK
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.2.1 Code Definitions
#[bitfield(u32)]
pub struct InfoMask {
    sys_font: bool,
    sys_size: bool,
    sys_style: bool,
    #[bits(access = RO)]
    reserved0: bool,
    sys_fore_color: bool,
    sys_back_color: bool,
    #[bits(6, access = RO)]
    reserved1: u8,
    resize: bool,
    restyle: bool,
    #[bits(2, access = RO)]
    reserved2: u8,
    any_font: bool,
    any_size: bool,
    any_style: bool,
    #[bits(13, access = RO)]
    reserved3: u16,
}

/// # EFI_IMAGE_OUTPUT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.2.1 Code Definitions
#[repr(C)]
pub struct ImageOutput<'a> {
    width: u16,
    height: u16,
    image: Image<'a>,
}

impl ImageOutput<'_> {
    fn bitmap(&self) -> &[graphics_output::BltPixel] {
        let bitmap: *const graphics_output::BltPixel = unsafe {
            self.image.bitmap
        } as *const graphics_output::BltPixel;
        let length: usize = self.width as usize * self.height as usize;
        unsafe {
            slice::from_raw_parts(bitmap, length)
        }
    }

    fn pixel(&self, x: usize, y: usize) -> graphics_output::BltPixel {
        self.bitmap()[self.width as usize * y + x].clone()
    }
}

#[repr(C)]
union Image<'a> {
    bitmap: &'a graphics_output::BltPixel,
    screen: &'a graphics_output::Protocol<'a>,
}

/// # EFI_FONT_INFO
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.3 String Protocol
#[repr(C)]
pub struct Info<'a> {
    style: Style,
    size: u16,
    name: char16::NullTerminatedString<'a>,
}

#[derive(Debug)]
pub struct Font<'a> {
    display_info: &'a DisplayInfo<'a>,
    character2coordinates2is_foreground: BTreeMap<char, BTreeMap<graphics_output::Coordinates, bool>>,
}

impl<'a> Font<'a> {
    pub fn new(display_info: &'a DisplayInfo<'a>, character2coordinates2is_foreground: BTreeMap<char, BTreeMap<graphics_output::Coordinates, bool>>) -> Self {
        Self {
            display_info,
            character2coordinates2is_foreground,
        }
    }
}

pub struct FontIterator<'a> {
    protocol: &'a Protocol,
    handle: Handle<'a>,
}

impl<'a> From<&'a Protocol> for FontIterator<'a> {
    fn from(protocol: &'a Protocol) -> Self {
        let handle: Handle<'a> = null();
        Self {
            protocol,
            handle,
        }
    }
}

impl<'a> Iterator for FontIterator<'a> {
    type Item = &'a DisplayInfo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let string_info_in: Self::Item = null();
        let mut string_info_out: Self::Item = null();
        let string = EfiString::null();
        (self.protocol.get_font_info)(self.protocol, &mut self.handle, string_info_in, &mut string_info_out, string)
            .result()
            .ok()
            .and_then(|_| (!self.handle .is_null()).then_some(string_info_out))
    }
}

