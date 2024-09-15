use pnet::datalink::{self, Channel::Ethernet};
use std::thread;

/// # Multi-Interface Network Packet Capture Program with Thread IDs
///
/// This program captures and displays network packets from **all available network interfaces** using the `pnet` crate.
/// It spawns a separate thread for each network interface to monitor packets concurrently, and prints the thread ID responsible
/// for capturing each packet, along with the packet's size and content in hexadecimal format.
///
/// ## Example
///
/// To run the program:
/// ```bash
/// sudo cargo run
/// ```
/// 
/// ## Notes:
/// - This program only supports Ethernet-based interfaces.
/// - **Administrative privileges** are required to run the program due to the need for access to network interfaces.
///
/// ## Error Handling
/// The program will handle errors in the following ways:
/// 1. If creating a channel for an interface fails, an error message is printed, but other interfaces will continue capturing packets.
/// 2. The program panics if a thread fails during execution.
///
/// ## Mermaid Diagram
///
/// The following diagram shows the flow of the program:
///
/// ```mermaid
/// graph TD;
///     A[Start] --> B{Are there interfaces?};
///     B -- Yes --> C[Spawn thread for each interface];
///     C --> D[Open channel for each interface];
///     D --> E[Capture packets in loop];
///     E --> F[Display packet data with thread ID];
///     F --> E;
///     B -- No --> G[Exit with error];
/// ```
/// 
/// ## Dependencies
///
/// Add the following dependency to your `Cargo.toml`:
/// ```toml
/// [dependencies]
/// pnet = "0.35.0"
///
/// [dev-dependencies]
/// aquamarine = "0.5.0"
/// ```
///
/// ## Code Overview
///
/// This code demonstrates spawning a thread for each network interface, capturing network packets concurrently, and displaying
/// the thread ID alongside each packet:
#[cfg_attr(doc, aquamarine::aquamarine)]
fn main() {
    // Retrieve the list of available network interfaces
    let interfaces = datalink::interfaces();

    // Spawn a thread for each interface
    let mut handles = Vec::new();

    for interface in interfaces {
        // Clone the interface to move into the thread
        let iface = interface.clone();

        // Spawn a new thread to handle packet capture for this interface
        let handle = thread::spawn(move || {
            // Open a channel on the selected interface
            let (_, mut rx) = match datalink::channel(&iface, Default::default()) {
                Ok(Ethernet(_tx, rx)) => (_tx, rx),
                Ok(_) => {
                    eprintln!("This program only supports Ethernet interfaces.");
                    return;
                }
                Err(e) => {
                    eprintln!("Failed to create channel on {}: {}", iface.name, e);
                    return;
                }
            };

            println!("Capturing packets on interface: {}", iface.name);

            // Infinite loop to capture and display packets
            loop {
                match rx.next() {
                    Ok(packet) => {
                        // Get the current thread ID
                        let thread_id = thread::current().id();

                        // Display the thread ID, packet size, and the name of the interface
                        println!(
                            "[Thread {:?}] Packet captured: {} bytes on interface {}",
                            thread_id,
                            packet.len(),
                            iface.name
                        );
                        for byte in packet {
                            print!("{:02x} ", byte);
                        }
                        println!();
                    }
                    Err(e) => {
                        eprintln!("Error capturing packet on {}: {}", iface.name, e);
                    }
                }
            }
        });

        // Store the handle to join later
        handles.push(handle);
    }

    // Join all threads to ensure the main thread waits for all of them to finish
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
