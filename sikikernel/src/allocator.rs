use bitvec::macros::internal::funty::Fundamental;
use bitvec::ptr::BitRef;
use bitvec::vec;
use bitvec::{bitarr, bits, BitArr};
use core::panic;
use core::{
    alloc::{GlobalAlloc, Layout},
    cell::{Cell, UnsafeCell},
    cmp, ptr,
};
use lib::{MemoryMap, MemoryType};
use num::Integer;

use crate::print_serial;
use crate::write::write_to;

const ALLOC_FRAME_SIZE: usize = 4096;
const ALLOC_FRAME_NUM: usize = 4096;

// グローバルメモリアロケータの宣言
#[global_allocator]
pub static ALLOC: SimpleAlloc = SimpleAlloc {
    start: UnsafeCell::new(0x0),
    end: UnsafeCell::new(0x0),
    total_pages: UnsafeCell::new(0x0),
    memory_frame: UnsafeCell::new(MemoryFrame {
        using_flag: bitarr![0; ALLOC_FRAME_NUM],
        once_frame_size: ALLOC_FRAME_SIZE,
        frame_num: ALLOC_FRAME_NUM,
        all_memory_size: ALLOC_FRAME_SIZE * ALLOC_FRAME_SIZE,
        offset: 0,
    }),
    initialized: UnsafeCell::new(false),
};

// *シングル*コアシステム用のポインタを増加するだけのアロケータ
pub struct SimpleAlloc {
    start: UnsafeCell<usize>,
    end: UnsafeCell<usize>,
    total_pages: UnsafeCell<usize>,
    memory_frame: UnsafeCell<MemoryFrame>,
    initialized: UnsafeCell<bool>,
}

impl SimpleAlloc {
    pub fn initialize(&self, memory_map: MemoryMap) {
        let mut alloc_start: usize = 0;
        let mut alloc_end: usize = 0;
        let mut total_pages: usize = 0;

        for i in 0..memory_map.len {
            if memory_map.map[i].memory_type != MemoryType::CONVENTIONAL {
                continue;
            }
            alloc_start = memory_map.map[i].physical_start as usize;
            alloc_end = memory_map.map[i + 1].physical_start as usize - 1;
            total_pages += memory_map.map[i].number_of_pages as usize;
        }

        unsafe {
            *self.start.get() = alloc_start;
            *self.end.get() = alloc_end;
            (*self.memory_frame.get()).offset = alloc_start;
            *self.initialized.get() = true;
        }
    }
}

unsafe impl Sync for SimpleAlloc {}

unsafe impl GlobalAlloc for SimpleAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if !*self.initialized.get() {
            panic!("Alloc is not initialized!!")
        }

        // let ptr = (self
        //     .memory_frame
        //     .get()
        //     .as_mut()
        //     .unwrap()
        //     .use_frame_with_physical_size(layout.align())
        //     .unwrap()) as *mut u8;
        // let mut buf = [0u8; 256];
        // let _s: &str = write_to::show(
        //     &mut buf,
        //     format_args!("layout: {}, addr: {:0x}\n", layout.align(), ptr.addr()),
        // )
        // .unwrap();
        // print_serial(_s);
        // ptr

        (self
            .memory_frame
            .get()
            .as_mut()
            .unwrap()
            .use_frame_with_physical_size(layout.align())
            .unwrap()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.memory_frame
            .get()
            .as_mut()
            .unwrap()
            .free_frame_with_physical_address(ptr.addr() as usize, layout.align())
    }
}

type UsingFlag = BitArr!(for ALLOC_FRAME_NUM);

#[derive(Debug, Default)]
pub struct MemoryFrame {
    using_flag: UsingFlag,
    once_frame_size: usize,
    frame_num: usize,
    all_memory_size: usize,
    offset: usize,
}

impl MemoryFrame {
    pub fn use_frame(&mut self, size: usize) -> Result<usize, &'static str> {
        let mut search_index = 0;
        let mut found_size = 0;
        loop {
            if self.get_flag(search_index).unwrap() == false {
                found_size += 1;
            } else {
                found_size = 0;
            }
            if found_size == size {
                break;
            }
            if search_index == self.frame_num {
                return Err("Memory not have enough of space");
            }
            search_index += 1;
        }
        let start = search_index - (found_size - 1);
        let end = search_index + 1;

        for i in start..end {
            self.set_flag(i, true);
        }

        let mut buf = [0u8; 256];
        let _s: &str = write_to::show(
            &mut buf,
            format_args!("alloc start index: {}, end index: {}\n", start, end),
        )
        .unwrap();
        print_serial(_s);

        // for i in self.using_flag.iter() {
        //     let mut buf = [0u8; 256];
        //     let _s: &str = write_to::show(&mut buf, format_args!("{}, ", i)).unwrap();
        //     print_serial(_s);
        // }

        Ok(start)
    }

    pub fn use_frame_with_physical_size(&mut self, size: usize) -> Result<usize, &'static str> {
        // 必要なフレーム数をメモリサイズから計算
        let need_frame_size = size.div_ceil(&self.once_frame_size);

        Ok(self.use_frame(need_frame_size)? * self.once_frame_size + self.offset)
    }

    pub fn free_frame(&mut self, index: usize, size: usize) {
        let start = index;
        let end = index + size;

        let mut buf = [0u8; 256];
        let _s: &str = write_to::show(
            &mut buf,
            format_args!("dealloc start index: {}, end index: {}\n", start, end),
        )
        .unwrap();
        print_serial(_s);

        for i in start..end {
            self.set_flag(i, false);
        }
    }

    pub fn free_frame_with_physical_address(&mut self, addr: usize, size: usize) {
        // メモリアドレスがフレーム数にあっていない値が入っていた場合パニックを起こす
        if !self.is_address_in_regulation(addr) {
            panic!("Address is not in regulation")
        }

        // フレームのインデックスとサイズをメモリサイズから計算
        let frame_index = (addr - self.offset) / self.once_frame_size;
        let frame_size = size.div_ceil(&self.once_frame_size);

        self.free_frame(frame_index, frame_size);
    }

    pub fn set_offset_addr(&mut self, offset: usize) {
        self.offset = offset
    }

    fn set_flag(&mut self, index: usize, value: bool) {
        self.using_flag.set(index, value)
    }

    fn get_flag(&self, index: usize) -> Option<BitRef<'_>> {
        self.using_flag.get(index)
    }

    fn is_address_in_regulation(&self, addr: usize) -> bool {
        addr % self.once_frame_size == 0
    }
}
