#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(lang_items)]

extern crate alloc;

use core::{arch::asm, cell::RefCell, panic::PanicInfo};

mod allocator;

use alloc::vec;
use critical_section::Mutex;
use once_cell::sync::Lazy;

use uart_16550::SerialPort;

use lib::{MemoryType, SikiOSArguments};

mod ascii_font;
mod critical_section_impl;
mod drivers;
mod graphics;
mod write;

use drivers::pci::pci::*;
use graphics::*;
use write::*;

const SERIAL_IO_PORT: u16 = 0x3F8;

static SERIAL_PORT: Lazy<Mutex<RefCell<SerialPort>>> = Lazy::new(|| {
    let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
    serial_port.init();
    Mutex::new(RefCell::new(serial_port))
});

fn print_serial(s: &str) {
    for i in s.as_bytes() {
        critical_section::with(|cs| SERIAL_PORT.borrow_ref_mut(cs).send(*i))
    }
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_serial("kernel is panic!!!\n");

    if let Some(error) = _info.payload().downcast_ref::<&str>() {
        let mut buf = [0u8; 256];
        let _s: &str = write_to::show(&mut buf, format_args!("{}\n", error)).unwrap();
        print_serial(_s);
    } else {
        print_serial("no info\n");
    }

    if let Some(location) = _info.location() {
        let mut buf = [0u8; 256];
        let _s: &str = write_to::show(
            &mut buf,
            format_args!("{}, {}\n", location.file(), location.line()),
        )
        .unwrap();
        print_serial(_s);
    } else {
        print_serial("no info\n");
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[no_mangle] // don't mangle the name of this function
pub extern "sysv64" fn _kernel_main(args: &SikiOSArguments) -> ! {
    let mut graphics = Graphics {
        frame_buffer_info: args.frame_buffer_info,
        mode_info: args.mode_info,
    };

    let (width, height) = graphics.get_resolve();

    print_serial("Hello, World!!!!!!\n");

    graphics.draw_rect(0, 0, width, height, Color(0, 0, 0));

    graphics.draw_rect(10, 10, 20, 20, Color(255, 255, 255));
    graphics.draw_fonts(40, 40, "Hello, World", Color(255, 0, 255));

    for i in 0..args.memory_map.len {
        let mut buf = [0u8; 256];
        let _s: &str = write_to::show(
            &mut buf,
            format_args!(
                "{}, {:?}, {:08x}, {:x}, {:x}\n",
                i,
                args.memory_map.memory_map[i].memory_type,
                args.memory_map.memory_map[i].physical_start,
                args.memory_map.memory_map[i].number_of_pages,
                args.memory_map.memory_map[i].attribute
            ),
        )
        .unwrap();
        print_serial(_s);
        graphics.draw_fonts(40, 60 + i as u32 * 20, _s, Color(255, 255, 255));
    }

    let mut total_pages = 0;
    for i in 0..args.memory_map.len {
        if args.memory_map.memory_map[i].memory_type != MemoryType::CONVENTIONAL {
            continue;
        }

        let mut buf = [0u8; 256];
        let _s: &str = write_to::show(
            &mut buf,
            format_args!(
                "{}, {:?}, {:08x}, {:x}, {:x}\n",
                i,
                args.memory_map.memory_map[i].memory_type,
                args.memory_map.memory_map[i].physical_start,
                args.memory_map.memory_map[i].number_of_pages,
                args.memory_map.memory_map[i].attribute
            ),
        )
        .unwrap();
        print_serial(_s);

        total_pages += args.memory_map.memory_map[i].number_of_pages;
    }

    let mut buf = [0u8; 256];
    let _s: &str = write_to::show(
        &mut buf,
        format_args!("total: {}Mib\n", total_pages * 4096 / 1024 / 1024),
    )
    .unwrap();
    print_serial(_s);

    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    v.push(2);

    for a in v.into_iter() {
        let mut buf = [0u8; 256];
        let _s: &str = write_to::show(&mut buf, format_args!("{}\n", a)).unwrap();
        print_serial(_s);
    }

    loop {
        unsafe { asm!("hlt") }
    }

    let mut pci = PCI::new();
    pci.initialize();

    for (i, pci_device) in pci.devices.iter().enumerate() {
        if i >= pci.device_index {
            continue;
        }

        let mut buf = [0u8; 128];
        let _s: &str = write_to::show(
            &mut buf,
            format_args!(
                "bus: {}, device: {}, function: {}, vendor_id: {}, base_class: {}, header_type: {}",
                pci_device.bus,
                pci_device.device,
                pci_device.function,
                pci_device.pci_configuration.vendor_id,
                pci_device.pci_configuration.base_class,
                pci_device.pci_configuration.header_type
            ),
        )
        .unwrap();

        graphics.draw_fonts(40, 60 + i as u32 * 20, _s, Color(255, 255, 255));
    }

    loop {}
}
