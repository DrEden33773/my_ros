#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ember_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use ember_os::allocator::HEAP_SIZE;

entry_point!(main);

#[no_mangle]
fn main(boot_info: &'static BootInfo) -> ! {
  ember_os::minimum_init(boot_info);
  test_main();
  ember_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  ember_os::test_panic_handler(info)
}

#[test_case]
fn simple_allocation() {
  let heap_value_1 = Box::new(41);
  let heap_value_2 = Box::new(13);
  assert_eq!(*heap_value_1, 41);
  assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn large_vec() {
  let n = 1000;
  let vec = (0..n).collect::<Vec<_>>();
  assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
  const DIV: usize = 1;
  (0..HEAP_SIZE / DIV).for_each(|i| {
    let x = Box::new(i); // new
    assert_eq!(*x, i);
  });
}

#[test_case]
fn rc_vec() {
  let origin_vec = vec![1, 2, 3];
  let rc_vec = Rc::new(origin_vec);
  assert_eq!(Rc::strong_count(&rc_vec), 1);
  let cloned_ref = rc_vec.clone();
  assert_eq!(Rc::strong_count(&cloned_ref), 2);
  core::mem::drop(cloned_ref);
  assert_eq!(Rc::strong_count(&rc_vec), 1);
}

/// Failed <|`iff`|> BumpAllocator
#[test_case]
fn many_boxes_long_lived() {
  let long_lived = Box::new(1); // new
  const DIV: usize = 1;
  (0..HEAP_SIZE / DIV).for_each(|i| {
    let x = Box::new(i); // new
    assert_eq!(*x, i);
  });
  assert_eq!(*long_lived, 1);
}
