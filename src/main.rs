
#[link(name="EMBEDDED_DATA", kind="static")]
extern "C" {
    static _binary_EMBEDDED_DATA_start: *const u8;
    static _binary_EMBEDDED_DATA_len: usize;
}

fn embedded_data() -> &'static [u8] {
    unsafe {
        std::slice::from_raw_parts(
            _binary_EMBEDDED_DATA_start,
            _binary_EMBEDDED_DATA_len
        )
    }
}

fn main() {
    let data = embedded_data();
    println!("data is at {:p} and {} bytes long", data.as_ptr(), data.len());
}