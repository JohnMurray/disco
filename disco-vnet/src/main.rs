/// # virtual-interface experiment
/// 
/// The goal here is to create a super simple virtual interface that can intercept
/// all local network traffic (perhaps with some help from iptables and friends)
///
/// ## do next
/// 
///   [ ] create a tun/tap interface and successfully init the sockets and such
///   [ ] try to read some packets sent to the assigned IP address
///   [ ] try to reach the IP from some other node on the network

fn main() {
    println!("Hello, world!");
}
