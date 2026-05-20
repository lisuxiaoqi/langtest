#![feature(allocator_api)]
use std::alloc::alloc;
use std::alloc::dealloc;
use std::alloc::handle_alloc_error;
use std::alloc::realloc;
use std::alloc::Allocator;
use std::alloc::Layout;
use std::ptr;

#[test]
fn test_alloc() {
    unsafe {
        let layout = Layout::new::<u64>();
        let ptr = alloc(layout);
        println!("{:?}", ptr);
        dealloc(ptr, layout);
    }
}

#[test]
fn test_vec() {
    struct MyVec<T> {
        len: usize,
        cap: usize,
        ptr: *mut T,
    }

    impl<T> MyVec<T> {
        fn new(cap: usize) -> Self {
            //handle ZST
            if size_of::<T>() == 0 {
                return Self {
                    len: 0,
                    cap: usize::MAX,
                    ptr: ptr::NonNull::dangling().as_ptr(),
                };
            }

            //handle cap is zero
            if cap == 0 {
                return Self {
                    cap: 0,
                    len: 0,
                    ptr: ptr::NonNull::dangling().as_ptr(),
                };
            }

            let layout = Layout::array::<T>(cap).unwrap();
            let ptr = unsafe { alloc(layout) as *mut T };
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            Self { len: 0, cap, ptr }
        }

        fn push(&mut self, elem: T) {
            if std::mem::size_of::<T>() == 0 {
                self.len += 1;
                return;
            }
            if self.len + 1 > self.cap {
                let new_cap = if self.cap == 0 {
                    4
                } else {
                    self.cap.checked_mul(2).expect("some error")
                };
                unsafe {
                    let layout = Layout::array::<T>(new_cap).unwrap();
                    let ptr = if self.cap == 0 {
                        alloc(layout) as *mut T
                    } else {
                        let old_layout = Layout::array::<T>(self.cap).unwrap();
                        realloc(self.ptr as *mut u8, old_layout, new_cap) as *mut T
                    };
                    //                    let ptr = alloc(layout) as *mut T;
                    //                    if ptr.is_null() {
                    //                        handle_alloc_error(layout);
                    //                    }
                    //                    //memory move
                    //                    if self.cap > 0 {
                    //                        ptr::copy_nonoverlapping(self.ptr, ptr, self.len);
                    //
                    //                        //release old memory
                    //                        let old_layout = Layout::array::<T>(self.cap).unwrap();
                    //                        dealloc(self.ptr as *mut u8, old_layout);
                    //                    }
                    //
                    // reset ptr
                    if ptr.is_null() {
                        handle_alloc_error(layout);
                    }
                    self.ptr = ptr;
                    self.cap = new_cap;
                }
            }
            unsafe {
                self.ptr.add(self.len).write(elem);
            }
            self.len += 1;
        }
    }

    impl<T> Drop for MyVec<T> {
        fn drop(&mut self) {
            unsafe {
                //release elem
                for i in 0..self.len {
                    ptr::drop_in_place(self.ptr.add(i));
                }
            }
            if std::mem::size_of::<T>() > 0 && self.cap > 0 {
                //release MyVec
                let layout = Layout::array::<T>(self.cap).unwrap();
                unsafe {
                    dealloc(self.ptr as *mut u8, layout);
                }
            }
        }
    }
}

#[test]
fn test_alloc_trait() {
    //Bump Allocator分配内存很快，很简单，但释放内存是个问题，因为它的cur_ptr是永远向前移动的，已经释放了的内存无法处理
    struct BumpAllocator {
        start_ptr: *mut u8,
        end_ptr: *mut u8,
        cur_ptr: *mut u8,
    }

    impl BumpAllocator {
        fn allocate(&mut self, layout: Layout) -> *mut u8 {
            let align = layout.align();
            let size = layout.size();
            let cur = self.cur_ptr as usize;
            let aligned = (cur + align - 1usize) & !(align - 1usize);

            let alloc_end = match aligned.checked_add(size) {
                Some(end) => end,
                None => return ptr::null_mut(),
            };
            if alloc_end > self.end_ptr as usize {
                return ptr::null_mut();
            }
            self.cur_ptr = alloc_end as *mut u8;
            aligned as *mut u8
        }
    }
}
