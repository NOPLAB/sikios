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

use alloc::vec::Vec;

use lib::{FrameBufferInfo, MemoryDescriptor, MemoryMap, ModeInfo};
use lib::{SikiOSArguments, MEMORY_MAP_SIZE};

use goblin::elf::{self};

use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::media::file::Directory;
use uefi::proto::media::file::RegularFile;
use uefi::table::boot::MemoryMapIter;
use uefi::table::boot::MemoryMapSize;
use uefi::{
    global_allocator::exit_boot_services,
    prelude::*,
    proto::media::file::{File, FileAttribute, FileInfo},
    table::boot::MemoryType,
};
use uefi_services::*;

fn get_memory_map_size(boot_services: &BootServices) -> MemoryMapSize {
    boot_services.memory_map_size()
}

// fn get_memory_map<'a>(
//     boot_services: &'a BootServices,
//     buffer: &'a mut Vec<u8>,
// ) -> MemoryMapIter<'a> {
//     println!("Get Memory Map");

//     let (_, memory_map_iter) = boot_services.memory_map(buffer).unwrap();

//     memory_map_iter
// }

fn get_memory_map<'a>(boot_services: &'a BootServices, buffer: &'a mut [u8]) -> MemoryMapIter<'a> {
    println!("Get Memory Map");

    let (_, memory_map_iter) = boot_services.memory_map(buffer).unwrap();

    memory_map_iter
}

fn print_memory_map(memory_map_iter: &MemoryMapIter) {
    println!("Print Memory Map");
    for (i, d) in memory_map_iter.clone().enumerate() {
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
}

fn save_memory_map(memory_map_iter: &MemoryMapIter, dir: &mut Directory) {
    println!("Save Memory Map");

    let mut memorymap_file = dir
        .open(
            cstr16!("\\memorymap"),
            uefi::proto::media::file::FileMode::CreateReadWrite,
            FileAttribute::from_bits(0).unwrap(),
        )
        .unwrap()
        .into_regular_file()
        .unwrap();

    memorymap_file.write("MemoryMap \n".as_bytes()).unwrap();
    for (i, d) in memory_map_iter.clone().enumerate() {
        let line = format!(
            "{}, {:x}, {:?}, {:08x}, {:x}, {:x}\n",
            i,
            d.ty.0,
            d.ty,
            d.phys_start,
            d.page_count,
            d.att.bits() & 0xfffff
        );
        memorymap_file.write(line.as_bytes()).unwrap();
    }
    memorymap_file.flush().unwrap();

    println!("Saved Memory Map");
}

fn load_file(dir: &mut Directory, file_name: &uefi::CStr16) -> RegularFile {
    // ファイルの情報を取得
    dir.open(
        file_name,
        uefi::proto::media::file::FileMode::Read,
        FileAttribute::from_bits(0).unwrap(),
    )
    .unwrap()
    .into_regular_file()
    .unwrap()
}

fn entry_kernel(entry: u64, args: &SikiOSArguments) {
    let _start: extern "sysv64" fn(args: &SikiOSArguments) = unsafe { mem::transmute(entry) };

    println!("Exit Boot Services");
    exit_boot_services();

    println!("Enter Entry Point");

    _start(args);
}

#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    println!("Hello World");

    let boot_services = system_table.boot_services();
    let mut simple_file_system = boot_services.get_image_file_system(handle).unwrap();
    let mut root_dir = simple_file_system.open_volume().unwrap();

    let memory_map_size = get_memory_map_size(boot_services).map_size;
    print!("Memory Map Size: {}\n", memory_map_size + 1024);
    // let mut memory_map_buffer = vec![0 as u8; memory_map_size + 1024];
    let mut memory_map_buffer = [0 as u8; 10240];
    let memory_map_iter = get_memory_map(boot_services, &mut memory_map_buffer);
    print_memory_map(&memory_map_iter);
    save_memory_map(&memory_map_iter, &mut root_dir);

    println!("Load Kernel");

    let mut elf_file = load_file(&mut root_dir, cstr16!("\\kernel.elf"));

    // ファイルサイズを取得
    let elf_file_info = elf_file.get_boxed_info::<FileInfo>().unwrap();
    let elf_file_size = elf_file_info.file_size() as usize;

    println!("Kernel File Size: 0x{:x}", elf_file_size);

    // バッファを作成
    let mut elf_buffer = vec![0; elf_file_size];

    // バッファに読み込み
    elf_file.read(&mut elf_buffer).unwrap();

    // goblinに変換
    let elf = elf::Elf::parse(&elf_buffer).unwrap();

    // ロードする位置の最小値と最大値を求める destは目標の位置という意味
    let mut dest_first = usize::MAX;
    let mut dest_last = 0;
    for ph in elf.program_headers.iter() {
        if ph.p_type != elf::program_header::PT_LOAD {
            continue;
        }
        dest_first = dest_first.min(ph.p_vaddr as usize);
        dest_last = dest_last.max((ph.p_vaddr + ph.p_memsz) as usize);
    }

    let load_size = dest_last as usize - dest_first;
    let n_of_pages = (load_size + 0xfff) / 0x1000;

    println!(
        "Kernel first: 0x{:x}, last: 0x{:x}, pages: {}",
        dest_first, dest_last, n_of_pages
    );

    // メモリを確保
    let _kernel_physical_addr = boot_services
        .allocate_pages(
            uefi::table::boot::AllocateType::Address(dest_first as u64),
            MemoryType::LOADER_DATA,
            n_of_pages,
        )
        .unwrap();

    println!("Kernel physical addr: 0x{:x}", _kernel_physical_addr);

    // 内容をコピー
    for ph in elf.program_headers.iter() {
        if ph.p_type != elf::program_header::PT_LOAD {
            continue;
        }
        let ofs = ph.p_offset as usize;
        let fsize = ph.p_filesz as usize;
        let msize = ph.p_memsz as usize;
        let dest = unsafe { from_raw_parts_mut(ph.p_vaddr as *mut u8, msize) };
        dest[..fsize].copy_from_slice(&elf_buffer[ofs..ofs + fsize]);
        dest[fsize..].fill(0);
    }

    println!("Entry Point: 0x{:x}", elf.entry);

    // let _gop_handle = _boot_services
    //     .get_handle_for_protocol::<GraphicsOutput>()
    //     .unwrap();

    // let mut _gop = _boot_services
    //     .open_protocol_exclusive::<GraphicsOutput>(_gop_handle)
    //     .unwrap();

    let graphics_output: &mut GraphicsOutput = unsafe {
        boot_services
            .locate_protocol::<GraphicsOutput>()
            .unwrap()
            .get()
            .as_mut()
            .unwrap()
    };

    let mut mode_info: ModeInfo = graphics_output.current_mode_info().into();
    println!("H: {}, V: {}", mode_info.hor_res, mode_info.ver_res);

    let mut frame_buffer = graphics_output.frame_buffer();
    let mut frame_buffer_info = FrameBufferInfo {
        fb: frame_buffer.as_mut_ptr(),
        size: frame_buffer.size(),
    };

    let mut memory_map: [MemoryDescriptor; MEMORY_MAP_SIZE] = [Default::default(); MEMORY_MAP_SIZE];

    for (i, value) in memory_map_iter.clone().enumerate() {
        memory_map[i].memory_type = value.ty.into();
        memory_map[i].physical_start = value.phys_start;
        memory_map[i].virtual_start = value.virt_start;
        memory_map[i].number_of_pages = value.page_count;
        memory_map[i].attribute = value.att.bits();
    }

    let args = SikiOSArguments {
        frame_buffer_info: frame_buffer_info,
        mode_info: mode_info,
        memory_map: MemoryMap {
            map: memory_map,
            len: memory_map_iter.len(),
        },
    };

    entry_kernel(elf.entry, &args);

    Status::SUCCESS
}
