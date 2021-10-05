#![allow(non_camel_case_types)]
pub type size_t = ::std::os::raw::c_ulong;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref STORAGE: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

#[no_mangle]
extern "C" fn store(vals: *const u8, len: size_t) {
    let a = unsafe { std::slice::from_raw_parts(vals, len as usize) };
    STORAGE.lock().unwrap().extend_from_slice(a);
    println!("store {:?}", a);
}

extern "C" {
    pub fn adc_callback(vals: *const u16, len: size_t);
    pub fn decompress(vals: *const u8, out: *mut u16, len_vals: size_t, len_out: size_t);
}

fn read_adc_data() -> Vec<u16> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let io = File::open("python/data.txt").unwrap();
    let br = BufReader::new(io);
    let lines: Vec<String> = br.lines()
        .filter_map(|s| s.ok())
        .collect();
    let nums: Vec<&str> = lines
        .iter()
        .flat_map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .collect();
    nums
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect()
}


fn main() {
    let vals = read_adc_data();
    let vals = &vals[0..200];
    let mut out = vec![0; vals.len()];
    let expect = vals.clone();

    unsafe {
        adc_callback(vals.as_ptr(), vals.len() as size_t);
        let buf = STORAGE.lock().unwrap();
        decompress(buf.as_ptr(), out.as_mut_ptr(), buf.len() as size_t, out.len() as size_t);
        assert_eq!(out.len(), expect.len())
    }
    let error_sum: i128 = expect.iter()
        .zip(&out)
        .map(|(a, b)| (*a as i128, *b as i128))
        .map(|(a, b)| (a-b)*(a-b))
        .sum();

    println!("adc_callback: {:?}", vals);
    println!("out: {:?}", out);
    println!("COMPRESSION RATIO: {}", STORAGE.lock().unwrap().len() as f64 / (expect.len() as f64 * 2.0));
    println!("MEAN ERROR: {}", error_sum as f64 / expect.len() as f64);
}
