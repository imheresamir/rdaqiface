extern crate libc;
use libc::{size_t, ssize_t};

#[link(name="daqiface", kind="static")]
extern { 
    fn daqiface_get_analog_input_scan(buf_ptr: *mut u8, buf_size: usize) -> ssize_t;
    fn daqiface_set_digital_output(digital_output_index: usize, state: bool) -> ssize_t;
    fn daqiface_get_digital_input(digital_input_index: usize, state: *mut bool) -> ssize_t;
}

pub fn get_analog_input_scan(scan_size: usize) -> Option<Vec<u8>> {
    unsafe {
        let mut scan_buf: Vec<u8> = Vec::with_capacity(scan_size);
        let retval = daqiface_get_analog_input_scan(scan_buf.as_mut_ptr(), scan_buf.capacity() as size_t);
        
        if retval != 0 {
            None
        } else {
            scan_buf.set_len(scan_buf.capacity());
            Some(scan_buf)
        }
    }
}

pub fn set_digital_output(digital_output_index: usize, state: bool) -> isize {
    unsafe {
       daqiface_set_digital_output(digital_output_index, state)
    }
}

pub fn get_digital_input(digital_input_index: usize) -> Option<bool> {
    unsafe {
        let mut state: bool = false;
        let retval = daqiface_get_digital_input(digital_input_index, &mut state);
        
        if retval != 0 {
            None
        } else {
            Some(state)
        }
    }
}

fn main() {
    let retval = set_digital_output(1, false);
    
    let digital_input_index = 3;
    let digital_input3_state = get_digital_input(digital_input_index).unwrap();
    println!("r: Got {} from digital in {}", digital_input3_state, digital_input_index); 

    let bytes_to_read = 10;
    let analog_input_scan = get_analog_input_scan(bytes_to_read).unwrap();
    println!("r: Got analog in slice:\n{:?}", analog_input_scan);
}
