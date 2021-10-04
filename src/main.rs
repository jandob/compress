
#![allow(non_camel_case_types)]
pub type size_t = ::std::os::raw::c_ulong;

#[no_mangle]
extern "C" fn store(vals: *const u16, len: size_t) {
    let a = unsafe { std::slice::from_raw_parts(vals, len as usize) };
    println!("store {:?}", a);
}

extern "C" {
    pub fn adc_callback(vals: *const u16, len: size_t);
    pub fn decompress(vals: *const u16, out: *mut u16, len: size_t);
}

fn main() {
    let vals: [u16;4] = [1,2,3,4];
    let mut out: [u16;4] = [0,0,0,0];
    let expect: [u16;4] = [1,2,3,4];

    println!("adc_callback: {:?}", vals);
    unsafe {
        adc_callback(vals.as_ptr(), vals.len() as size_t);
        decompress(vals.as_ptr(), out.as_mut_ptr(), vals.len() as size_t);
    }
    println!("out: {:?}", out);
    assert_eq!(out, expect)
}
