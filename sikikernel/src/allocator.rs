use core::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
    cmp, ptr,
};

// グローバルメモリアロケータの宣言
// ユーザはメモリ領域の`[0x2000_0100, 0x2000_0200]`がプログラムの他の部分で使用されないことを
// 保証しなければなりません
#[global_allocator]
static HEAP: SimpleAlloc = SimpleAlloc {
    head: UnsafeCell::new(0x100000),
    end: 0x3d0000,
};

// *シングル*コアシステム用のポインタを増加するだけのアロケータ
struct SimpleAlloc {
    head: UnsafeCell<usize>,
    end: usize,
}

unsafe impl Sync for SimpleAlloc {}

unsafe impl GlobalAlloc for SimpleAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let head = self.head.get();

        let align = layout.align();
        let res = *head % align;
        let start = if res == 0 { *head } else { *head + align - res };
        if start + align > self.end {
            // ヌルポインタはメモリ不足の状態を知らせます
            ptr::null_mut()
        } else {
            *head = start + align;
            start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // このアロケータはメモリを解放しません
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let head = self.head.get();

        let align = layout.align();
        let res = *head % align;
        let start = if res == 0 { *head } else { *head + align - res };
        if start + align > self.end {
            // ヌルポインタはメモリ不足の状態を知らせます
            ptr::null_mut()
        } else {
            *head = start + align;
            start as *mut u8
        }
    }
}
