use libc::c_int;

#[link_args="-L."]
extern mod runtime {
  fn run(callMeFromC: *u8) -> c_int;
}

fn main() {
  unsafe {
    runtime::run(callMeFromC);
  }
}

extern fn callMeFromC() -> *u8 {
  io::println("Hello from Rust");
  return ptr::null();
}

