
extern "C" {
    static _binary_EMBEDDED_DATA_start: u8;
    static _binary_EMBEDDED_DATA_end: u8;
}

fn embedded_data() -> &'static [u8] {
    unsafe {
        let start_ptr = (&_binary_EMBEDDED_DATA_start) as *const u8;
        let end_ptr = (&_binary_EMBEDDED_DATA_end) as *const u8;
        let len = end_ptr as usize - start_ptr as usize;
        std::slice::from_raw_parts(start_ptr, len)
    }
}

fn main() {
    let data = embedded_data();
    println!("data is at {:p} and {} bytes long", data.as_ptr(), data.len());
    println!("{}", std::str::from_utf8(data).unwrap());
}