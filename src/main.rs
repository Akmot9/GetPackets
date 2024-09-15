use pnet::datalink::{self, Channel::Ethernet};

/// # Network Packet Capture Program
///
/// This program captures and displays network packets passing through a specified network interface using the `pnet` crate.
/// It continuously monitors an interface, printing the size and content of each packet in hexadecimal format.
///
/// ## How It Works
/// 1. **Network Interface Selection**:
///    - The program selects a network interface by name (hardcoded in the code as `interface_name`).
///    - It fetches a list of available network interfaces and matches the one corresponding to `interface_name`.
///
/// 2. **Packet Capture**:
///    - A channel is opened on the selected interface using the `pnet::datalink::channel` method.
///    - The program enters an infinite loop, waiting to capture packets using the receiver (`rx`).
///
/// 3. **Packet Display**:
///    - Each captured packet's size and content are printed in hexadecimal format.
///
/// ## Mermaid Flow Diagram
///
/// This diagram outlines the program flow:
///
/// ```mermaid
/// %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#ffcc00', 'edgeLabelBackground':'#ffffff', 'tertiaryColor': '#fff' }}}%%
/// graph TD;
///     A[Start] --> B{Is the interface name valid?};
///     B -- Yes --> C[Open network interface channel];
///     B -- No --> F[Exit with error];
///     C --> D[Capture packets in a loop];
///     D --> E[Display packet size and content];
///     E --> D;
/// ```
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
/// - Administrative privileges are required to run the program due to the need for access to network interfaces.
///
/// ## Error Handling
/// The program will panic in the following cases:
/// 1. If the specified interface name is not found.
/// 2. If creating the channel for the interface fails.
///
/// ## Dependencies
///
/// Add the following dependencies to your `Cargo.toml`:
/// ```toml
/// [dependencies]
/// pnet = "0.35.0"
/// aquamarine = "0.5.0"
/// ```
///
/// ## Code Overview
/// This code demonstrates the functionality:
#[cfg_attr(doc, aquamarine::aquamarine)] // Enables the Mermaid diagram in rustdoc
fn main() {
    // Specify the network interface to capture packets from
    let interface_name = "wlp6s0";

    // Retrieve the list of available network interfaces
    let interfaces: Vec<datalink::NetworkInterface> = datalink::interfaces();

    let interface: datalink::NetworkInterface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("Network interface not found");

    // Open a channel on the selected interface
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_tx, rx)) => (_tx, rx),
        Ok(_) => panic!("This program only supports Ethernet interfaces."),
        Err(e) => panic!("Failed to create channel: {}", e),
    };

    println!("Capturing packets on interface: {}", &interface);

    // Infinite loop to capture and display packets
    loop {
        match rx.next() {
            Ok(packet) => {
                // Display the packet size and the name of the interface
                println!(
                    "Packet captured: {} bytes on interface {}",
                    packet.len(),
                    &interface,
                );
                for byte in packet {
                    print!("{:02x} ", byte);
                }
                println!();
            }
            Err(e) => {
                println!("Error capturing packet: {}", e);
            }
        }
    }
}
