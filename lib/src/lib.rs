#![crate_type = "lib"]
#![no_std]

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FrameBufferInfo {
    pub fb: *mut u8,
    pub size: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb = 0,
    Bgr,
    Bitmask,
    BltOnly,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PixelBitmask {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ModeInfo {
    pub version: u32, // must 0
    pub hor_res: u32,
    pub ver_res: u32,
    pub format: PixelFormat,
    pub mask: PixelBitmask,
    pub stride: u32,
}

#[cfg(feature = "uefi-feature")]
impl From<uefi::proto::console::gop::ModeInfo> for ModeInfo {
    fn from(item: uefi::proto::console::gop::ModeInfo) -> Self {
        let pixel_format = match item.pixel_format() {
            uefi::proto::console::gop::PixelFormat::Bgr => PixelFormat::Bgr,
            uefi::proto::console::gop::PixelFormat::Bitmask => PixelFormat::Bitmask,
            uefi::proto::console::gop::PixelFormat::BltOnly => PixelFormat::BltOnly,
            uefi::proto::console::gop::PixelFormat::Rgb => PixelFormat::Rgb,
        };

        ModeInfo {
            version: 0,
            hor_res: item.resolution().0 as u32,
            ver_res: item.resolution().1 as u32,
            format: pixel_format,
            mask: PixelBitmask {
                red: item.pixel_bitmask().unwrap().red,
                green: item.pixel_bitmask().unwrap().green,
                blue: item.pixel_bitmask().unwrap().blue,
                reserved: item.pixel_bitmask().unwrap().reserved,
            },
            stride: item.stride() as u32,
        }
    }
}
