mod libc_ext;
mod net;

fn main() {
    // create a raw socket for reading ethernet packets
    let socket = unsafe { net::init_raw_ethernet_socket() };

    // fetch all the hardware interfaces available and select one
    let interfaces = unsafe { net::list_ethernet_interfaces() };
    assert!(!interfaces.is_empty(), "No network interfaces discovered");
    let interface = prompt_select_interfaces(interfaces);
    println!("Selected interface: {}", interface);

    // bind the socket to the interface
    unsafe { net::bind_on_interface(socket, &interface) };

    // setup while loop to read from socket

    // write bytes out to screen in hex or something
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