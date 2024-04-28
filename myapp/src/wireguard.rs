use std::net::Ipv4Addr;
use wireguard_rs::{WireGuard, WireGuardConfig, WireGuardInterface};

fn main() {
    // Configuração da interface WireGuard
    let config = WireGuardConfig {
        private_key: "YOUR_PRIVATE_KEY".to_string(),
        listen_port: 51820,
        fwmark: None,
        replace_peers: false,
        peers: vec![],
    };

    // Criação da interface WireGuard
    let interface = WireGuardInterface::new("wg0".to_string(), config).unwrap();

    // Exemplo de como adicionar uma configuração de peer
    let peer_config = wireguard_rs::PeerConfig {
        public_key: "PEER_PUBLIC_KEY".to_string(),
        endpoint: Some("PEER_ENDPOINT".to_string()),
        allowed_ips: vec![Ipv4Addr::new(0, 0, 0, 0).into()],
        persistent_keepalive: None,
        preshared_key: None,
    };

    // Adicionando o peer à interface
    interface.add_peer(peer_config).unwrap();

    // Aqui você pode adicionar mais lógica para gerenciar a conexão, como iniciar e parar a interface

    println!("Interface WireGuard configurada com sucesso!");
}
