use uuid::Uuid;
use std::ffi::CString;

/// Generate a UUIDv7 and return as a C string
///
/// This function allocates memory for the UUID string.
/// SAS will handle the memory management when called through PROC PROTO.
#[no_mangle]
pub extern "C" fn uuidv7() -> *mut libc::c_char {
    // Generate UUIDv7
    let uuid = Uuid::now_v7();
    let uuid_string = uuid.to_string();
    
    // Convert to C string
    match CString::new(uuid_string) {
        Ok(c_str) => c_str.into_raw(), // Transfer ownership to C
        Err(_) => std::ptr::null_mut(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_uuidv7() {
        let ptr = uuidv7();
        assert!(!ptr.is_null());
        
        // Convert back to Rust string to verify format
        let c_str = unsafe { CStr::from_ptr(ptr) };
        let uuid_str = c_str.to_str().unwrap();
        
        // Validate UUID format
        assert_eq!(uuid_str.len(), 36);
        let parts: Vec<&str> = uuid_str.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
        
        // In our test, we need to free the memory
        unsafe { libc::free(ptr as *mut libc::c_void) };
    }
}