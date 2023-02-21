use crate::drivers::usb::xhci::*;

#[derive(Debug)]
pub struct USB {}

#[derive(Debug)]
pub struct USBDevice {
    xhci: Xhci,
}

// pub trait USBDriver {
//     fn initialize(&mut self) -> Result<(), USBDriverError>;
//     fn destroy(&mut self) -> Result<(), USBDriverError>;
// }

#[derive(Debug)]
pub enum USBDriverError {
    AnythingErr,
}

// impl USBDriver for Xhci {
//     fn initialize(&mut self) -> Result<(), USBDriverError> {
//         todo!()
//     }
//     fn destroy(&mut self) -> Result<(), USBDriverError> {
//         todo!()
//     }
// }
