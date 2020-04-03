use super::libc_ext;

type EthernetSocketDescriptor = i32;

/// A C-struct for fetching interface information from the kernel (missing
/// from the libc crate). The name is set by us and the union represents
/// raw bytes for what is a C-union to answer different questions we could
/// ask with `ioctl`
#[repr(C)]
pub struct IfReq {
    pub ifrn_name: [libc::c_char; libc::IF_NAMESIZE],
    pub union: [u8; 24],
}

impl IfReq {
    pub fn new(interface_name: &str) -> IfReq {
        // FIXME: this could definitely be better
        let mut name: [libc::c_char; libc::IF_NAMESIZE] = [0 as libc::c_char; libc::IF_NAMESIZE];
        for (i, byte) in interface_name.as_bytes().iter().enumerate() {
            name[i] = *byte as libc::c_char;
        }
        IfReq {
            ifrn_name: name,
            union: [0; 24],
        }
    }
}

/// Initialize a raw socket for reading ethernet packets. The socket
/// created will be for ALL ethernet packet types. Filtering if desired
/// will need to be done in user-space.
///
/// __Returns__ a valid socket descriptor. Failure to create a socket will
/// fail with an assertion/panic.
pub unsafe fn init_raw_ethernet_socket() -> EthernetSocketDescriptor {
    let socket = libc::socket(libc::PF_PACKET, libc::SOCK_RAW, libc_ext::ETH_P_ALL.to_be());
    libc_ext::assert(|| socket > 0, "create socket");

    socket
}

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

/// Bind a socket to an interface in order to read raw eithernet packets from
/// that interface.
/// 
/// ## Parameters
///  - __socket__    - Socket for raw ethernet to bind with
///  - __interface__ - Name of interface to bind on (to listen to ethernet packets from)
/// 
/// ## Returns
/// Nothing. However the function will assert (read panic) if it is unable to properly
/// bind on the interface.
pub unsafe fn bind_on_interface(socket: EthernetSocketDescriptor, interface: &str) -> () {
    // In order to properly bind for raw packets we need to bind the
    // socket to a particular interface. In order to bind to an interface
    // we need to turn the name into an index to set on our `sockaddr_ll` object.
    let mut ifr = IfReq::new(interface);
    let ret = libc::ioctl(socket, libc_ext::SIOCGIFINDEX, &mut ifr);
    libc_ext::assert(|| ret == 0, "retreiving interface index");

    // The `ifreq` struct from C is kind of a grab-bag. As such we need to
    // reconstruct our value from the union type by taking the first four
    // bytes into a u32. The union's value is determiend by the flag we pass
    // into ioctl above.
    let index: i32 = (ifr.union[3] as i32) << 24
        | (ifr.union[2] as i32) << 16
        | (ifr.union[1] as i32) << 8
        | (ifr.union[0] as i32);
    libc_ext::assert(|| index > 0, "retrieving interface index");
    println!("socket index: {}", index);

    let mut sock_addr: libc::sockaddr_ll = std::mem::zeroed();
    sock_addr.sll_family = libc::PF_PACKET as u16;
    sock_addr.sll_protocol = libc_ext::ETH_P_ALL.to_be() as u16;
    sock_addr.sll_ifindex = index;

    let sock_addr_ptr_ll = &mut sock_addr as *mut libc::sockaddr_ll;
    let sock_addr_ptr = sock_addr_ptr_ll as *mut libc::sockaddr;

    let size = std::mem::size_of::<libc::sockaddr_ll>();

    let ret = libc::bind(socket, sock_addr_ptr, size as u32);
    libc_ext::assert(|| ret == 0, "bind to raw socket");
}
