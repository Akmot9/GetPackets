use pnet::datalink::{self, Channel::Ethernet};
use std::env;

fn main() {
    // Récupère l'interface réseau à utiliser à partir des arguments de la ligne de commande
    let interface_name = env::args().nth(1).expect("Veuillez fournir le nom de l'interface réseau");

    // Cherche l'interface réseau en fonction du nom fourni
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("Interface réseau non trouvée");

    // Ouvre un canal sur l'interface sélectionnée
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Ce programme ne supporte que les interfaces Ethernet."),
        Err(e) => panic!("Impossible de créer le canal: {}", e),
    };

    println!("Capture des paquets sur l'interface: {}", interface);

    // Boucle infinie pour capturer et afficher les paquets
    loop {
        match rx.next() {
            Ok(packet) => {
                // Affiche la taille du paquet et le nom de l'interface
                println!(
                    "Paquet capturé: taille {} octets par l'interface {}",
                    packet.len(),
                    interface_name,
                );
                for byte in packet {
                    print!("{:02x} ", byte);
                }
                println!();
            }
            Err(e) => {
                println!("Erreur lors de la capture du paquet: {}", e);
            }
        }
    }
}
