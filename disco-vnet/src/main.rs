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

mod libc_ext;
mod net;

fn main() {
    println!("Select an interface to bridge:");
    let interface = prompt_select_interfaces(
        unsafe { net::list_ethernet_interfaces() }
    );
    println!("Selected: {}", interface);
}

fn prompt_select_interfaces(options: Vec<String>) -> String {
    use std::io::{stdin, stdout, Write};

    println!("Please select an interface option from below:");
    for (i, op) in options.iter().enumerate() {
        println!("    {}) {}", i + 1, op);
    }
    let _ = stdout().flush();

    // Get an answer from the user
    // FIXME: ugg... look at this mess
    let mut user_input = String::new();
    loop {
        print!("\nSelection: ");
        let _ = stdout().flush();

        match stdin().read_line(&mut user_input) {
            Ok(_) => {
                trim_stdin(&mut user_input);
                match user_input.parse::<usize>() {
                    Ok(index) => {
                        if index > options.len() || index == 0 {
                            println!(
                                "Please select an index from the above options, you entered {}",
                                index
                            )
                        } else {
                            break options[index - 1].clone();
                        }
                    }
                    _ => println!("Please make your selection with the numeric value"),
                }
            }
            _ => println!("Please input a numeric value"),
        }

        user_input.clear();
    }
}

fn trim_stdin(input: &mut String) {
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
}