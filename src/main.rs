
extern "C" {
    static _binary_EMBEDDED_DATA_start: u8;
    static _binary_EMBEDDED_DATA_size: ();
}

fn embedded_data() -> &'static [u8] {
    unsafe {
        std::slice::from_raw_parts(
            &_binary_EMBEDDED_DATA_start,
            (&_binary_EMBEDDED_DATA_size) as *const () as usize
        )
    }
}

fn main() {
    let data = embedded_data();
    println!("data is at {:p} and {} bytes long", data.as_ptr(), data.len());
    println!("{}", std::str::from_utf8(data).unwrap());
}