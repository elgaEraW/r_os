#![no_std]
#![no_main]

use core::panic::PanicInfo;
use r_os::{
  hlt_loop, serial_print, serial_println,
  testing::{exit_qemu, QemuExitCode},
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
  should_fail();
  serial_println!("[test did not panic]");
  exit_qemu(QemuExitCode::Failed);

  hlt_loop();
}

fn should_fail() {
  serial_print!("should_panic::should_fail...\t");
  assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  serial_println!("[ok]");
  exit_qemu(QemuExitCode::Success);

  hlt_loop();
}
