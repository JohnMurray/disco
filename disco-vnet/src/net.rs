use super::libc_ext;

/// Query all network interfaces and return a list of all interfaces that
/// are able to listen for ethernet packets.
/// 
/// __Returns__ a list of interface names as `Vec<String>`.
pub unsafe fn list_ethernet_interfaces() -> Vec<String> {
    use std::ffi::CStr;

    let mut ifa_ptr: *mut libc::ifaddrs = std::ptr::null_mut();
    let res: i32 = libc::getifaddrs(&mut ifa_ptr as *mut *mut libc::ifaddrs);
    libc_ext::assert(|| res == 0, "fetching network interfaces");

    let mut interface_names: Vec<String> = vec![];
    while ifa_ptr != std::ptr::null_mut() {
        if (*ifa_ptr).ifa_addr != std::ptr::null_mut()
            && (*(*ifa_ptr).ifa_addr).sa_family == (libc::PF_PACKET as u16)
        {
            let name = CStr::from_ptr((*ifa_ptr).ifa_name);
            match name.to_str() {
                Ok(c_str) => interface_names.push(String::from(c_str)),
                _ => (),
            }
        }
        ifa_ptr = (*ifa_ptr).ifa_next;
    }

    interface_names
}