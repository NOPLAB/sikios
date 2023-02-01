#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(ptr_metadata)]
#![allow(stable_features)]

#[macro_use]
extern crate alloc;

use core::mem;
use core::slice::from_raw_parts_mut;
use core::u8;

use alloc::vec::{self, Vec};
use goblin::elf::{self};
use uefi::proto::console::gop::FrameBuffer;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::SearchType;
use uefi::Identify;
use uefi::{
    global_allocator::exit_boot_services,
    prelude::*,
    proto::media::file::{File, FileAttribute, FileInfo},
    table::boot::MemoryType,
};
use uefi_services::*;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    println!("Hello World");

    let _boot_services = system_table.boot_services();

    println!("Get Memory Map");

    let memory_map_size = _boot_services.memory_map_size();

    let mut buffer = vec![0; memory_map_size.map_size + 1024];
    let (_, _memory_map_iter) = _boot_services.memory_map(&mut buffer).unwrap();

    println!("Print Memory Map");
    for (i, d) in _memory_map_iter.clone().enumerate() {
        let line = format!(
            "{}, {:x}, {:?}, {:08x}, {:x}, {:x}",
            i,
            d.ty.0,
            d.ty,
            d.phys_start,
            d.page_count,
            d.att.bits() & 0xfffff
        );
        println!("{}", line);
    }

    println!("Save Memory Map");

    let mut _simple_file_system = _boot_services.get_image_file_system(_handle).unwrap();
    let mut _root_dir = _simple_file_system.open_volume().unwrap();

    let mut _memorymap_file = _root_dir
        .open(
            cstr16!("\\memorymap"),
            uefi::proto::media::file::FileMode::CreateReadWrite,
            FileAttribute::from_bits(0).unwrap(),
        )
        .unwrap()
        .into_regular_file()
        .unwrap();

    _memorymap_file.write("MemoryMap \n".as_bytes()).unwrap();
    for (i, d) in _memory_map_iter.clone().enumerate() {
        let line = format!(
            "{}, {:x}, {:?}, {:08x}, {:x}, {:x}\n",
            i,
            d.ty.0,
            d.ty,
            d.phys_start,
            d.page_count,
            d.att.bits() & 0xfffff
        );
        _memorymap_file.write(line.as_bytes()).unwrap();
    }
    _memorymap_file.flush().unwrap();

    println!("Saved Memory Map");

    println!("Load Kernel");

    // ファイルの情報を取得
    let mut _elf_file = _root_dir
        .open(
            cstr16!("\\kernel.elf"),
            uefi::proto::media::file::FileMode::Read,
            FileAttribute::from_bits(0).unwrap(),
        )
        .unwrap()
        .into_regular_file()
        .unwrap();

    // ファイルサイズを取得
    let _elf_file_info = _elf_file.get_boxed_info::<FileInfo>().unwrap();
    let _elf_file_size = _elf_file_info.file_size() as usize;

    println!("Kernel File Size: {}", _elf_file_size);

    let mut _elf_buffer = vec![0; _elf_file_size];
    _elf_file.read(&mut _elf_buffer).unwrap();

    let elf = elf::Elf::parse(&_elf_buffer).unwrap();

    // ロードする位置の最小値と最大値を求める destは目標の位置という意味らしい
    let mut dest_first = usize::MAX; // 小さい方と比較するので変数に入る最大値で初期化
    let mut dest_last = 0;
    for ph in elf.program_headers.iter() {
        if (ph.p_type != elf::program_header::PT_LOAD) {
            continue;
        }
        dest_first = core::cmp::min(dest_first, ph.p_vaddr as usize);
        dest_last = core::cmp::max(dest_last, ph.p_vaddr + ph.p_memsz);
    }

    _boot_services.allocate_pages(
        uefi::table::boot::AllocateType::Address(dest_first as u64),
        MemoryType::LOADER_DATA,
        dest_last as usize,
    );

    for ph in elf.program_headers.iter() {
        if (ph.p_type != elf::program_header::PT_LOAD) {
            continue;
        }
        let ofs = ph.p_offset as usize;
        let fsize = ph.p_filesz as usize;
        let msize = ph.p_memsz as usize;
        let dest = unsafe { core::slice::from_raw_parts_mut(ph.p_vaddr as *mut u8, msize) };
        dest[..fsize].copy_from_slice(&_elf_buffer[ofs..ofs + fsize]);
        dest[fsize..].fill(0);
    }

    _boot_services.free_pool(dest_first as *mut u8);

    let _gop_handle = _boot_services
        .get_handle_for_protocol::<GraphicsOutput>()
        .unwrap();

    let mut _gop = _boot_services
        .open_protocol_exclusive::<GraphicsOutput>(_gop_handle)
        .unwrap();
    let _gop_info = _gop.current_mode_info();
    println!(
        "H: {}, V: {}",
        _gop_info.resolution().0,
        _gop_info.resolution().1
    );

    let mut _frame_buffer = _gop.frame_buffer();

    // for i in 0..2 {
    //     unsafe { from_raw_parts_mut(_frame_buffer.as_mut_ptr(), _frame_buffer.size())[i] = 255 }
    // }
    unsafe { from_raw_parts_mut(_frame_buffer.as_mut_ptr(), _frame_buffer.size())[0] = 255 }
    unsafe { from_raw_parts_mut(_frame_buffer.as_mut_ptr(), _frame_buffer.size())[1] = 0 }
    unsafe { from_raw_parts_mut(_frame_buffer.as_mut_ptr(), _frame_buffer.size())[2] = 0 }

    println!("Exit Boot Services");
    exit_boot_services();

    let KernelMain: extern "sysv64" fn() = unsafe { mem::transmute(elf.entry as usize) };
    KernelMain();

    loop {}

    Status::SUCCESS
}
