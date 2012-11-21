use libc::c_int;
use libc::c_char;

#[link_args="-L."]
extern mod runtime {
  fn run(argc: c_int, argv: **c_char, callMeFromC: *u8) -> c_int;
}

fn identity<T>(x: T) -> T { x }

fn main() {
  let args: ~[~str] = os::args();
  let argc: c_int = args.len() as c_int;
  unsafe {
    let argsVec: ~[*c_char] = args.map(|arg| { str::as_c_str(*arg, identity) });
    let argv: **c_char = vec::raw::to_ptr(argsVec);
    runtime::run(argc, argv, callMeFromC);
  }
}

extern fn callMeFromC() -> *u8 {
  io::println("Hello from Rust");
  return ptr::null();
}

