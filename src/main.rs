#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zefir_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use zefir_os::println;
use zefir_os::task::{executor::Executor, keyboard, Task};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use zefir_os::allocator;
    use zefir_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;
    println!("
    d88888D d88888b d88888b d888888b d8888b.      .d88b.  .d8888.
    YP  d8' 88'     88'       `88'   88  `8D     .8P  Y8. 88'  YP
       d8'  88ooooo 88ooo      88    88oobY'     88    88 `8bo.
      d8'   88~~~~~ 88~~~      88    88`8b       88    88   `Y8b.
     d8' db 88.     88        .88.   88 `88.     `8b  d8' db   8D
    d88888P Y88888P YP      Y888888P 88   YD      `Y88P'  `8888Y'
\n\n");
    
    println!("Operating system in Rust programming language.\n");
    println!("Project author: Pyzhov Viktor\n\n\n");

    zefir_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    // executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    zefir_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zefir_os::test_panic_handler(info)
}

// async fn async_number() -> u32 {
//     999
// }

// async fn example_task() {
//     let number = async_number().await;
//     println!("async number: {}", number);
// }

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
