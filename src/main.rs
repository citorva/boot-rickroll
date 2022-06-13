#![no_std]
#![no_main]

#![feature(core_intrinsics)]

mod serial;
mod music_player;
mod music;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use x86::io::{ inb, outb };

// Dynamically define later, frequency in MHz
static mut BASE_FREQUENCY : u64 = 3_000;

entry_point!(kernel_main);

fn definitely_sleep() -> ! {
    unsafe {
        x86::irq::disable();
        x86::halt();
    }

    loop {}
}

fn tick_elapsed() -> u64 {
    unsafe { x86::time::rdtsc() }
}

pub fn sleep_ticks(ticks : u64) {
    let to = tick_elapsed() + ticks;

    while to > tick_elapsed() {}
}

pub fn sleep(microseconds : u64) {
    sleep_ticks(microseconds * unsafe { BASE_FREQUENCY })
}

fn play_sound(frequency: f32) {
    let div = (1193180. / frequency) as u32;

    unsafe {
        // Configuring the PIT:
        // 10110110
        // │ │ │  └─▷ BCD/Binary mode: 16 bit binary
        // │ │ └────▷ Operating mode: Square wave generator
        // │ └──────▷ Access mode: lobyte/hibyte
        // └────────▷ Selected channel: Channel 2
        outb(0x43, 0b10110110);


        outb(0x42, (div & 0xFF) as u8);

        outb(0x42, (div >> 8 & 0xFF) as u8);
    }

    let tmp = unsafe { inb(0x61) };

    if tmp != (tmp | 3) {
        unsafe { outb(0x61, tmp | 3) };
    }
}

fn stop_sound() {
    let tmp = unsafe { inb(0x61) & 0xFC };

    unsafe { outb(0x61, tmp) };
}

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    music::music();

    definitely_sleep()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);

    for _ in 0..5 {
        play_sound(1000.);

        sleep_ticks(500_000_000);

        stop_sound();

        sleep_ticks(500_000_000);
    }

    definitely_sleep()
}