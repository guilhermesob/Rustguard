# Projeto de VPN em Rust

Este projeto tem como objetivo criar uma VPN (Virtual Private Network) simples usando a linguagem de programação Rust. A VPN irá criptografar o tráfego de rede entre o seu dispositivo e a internet, proporcionando uma camada adicional de segurança.

## Configuração do Ambiente de Desenvolvimento

1. **Instalação do Rust**: Verifique se o Rust está instalado no seu sistema executando `rustc --version` no terminal. Se não estiver instalado, você pode baixá-lo e instalá-lo a partir do site oficial do Rust: https://www.rust-lang.org/tools/install.

2. **Criação do Projeto Rust**: Navegue até o diretório onde deseja criar o projeto e execute os seguintes comandos:

```bash
cargo new vpn_project
cd vpn_project
```

## Adicionando Dependências

Adicione as seguintes dependências ao seu arquivo `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
rustls = "0.20"
```

## Implementação da VPN

A implementação da VPN envolve a criação de um servidor TCP que aceita conexões e configura um túnel seguro entre o cliente e o servidor. O código básico para iniciar o servidor e configurar o TLS está disponível no arquivo `main.rs`.

## Compilando e Executando

Para compilar e executar o projeto, use o seguinte comando:

```bash
cargo run
```

## Notas Finais

Este projeto é um ponto de partida para a criação de uma VPN em Rust. A implementação de uma VPN completa e segura é um projeto complexo que requer um entendimento profundo de redes, criptografia e segurança.

---

### Diagrama UML Básico

Para um projeto de VPN, um diagrama UML básico pode incluir os seguintes componentes:

- **Classe `VPNServer`**: Representa o servidor VPN.
 - **Atributos**:
    - `listener`: Um `TcpListener` para aceitar conexões.
    - `config`: Uma configuração TLS para criptografar as conexões.
 - **Métodos**:
    - `start()`: Inicia o servidor e começa a aceitar conexões.
    - `handle_connection(stream: TcpStream)`: Lida com uma conexão de cliente, criando um túnel seguro.

- **Classe `VPNClient`**: Representa o cliente VPN.
 - **Atributos**:
    - `stream`: Um `TcpStream` para a conexão com o servidor.
 - **Métodos**:
    - `connect(server_address: String)`: Conecta ao servidor VPN.
    - `send_data(data: Vec<u8>)`: Envia dados através do túnel VPN.
    - `receive_data()`: Recebe dados através do túnel VPN.

**versão**
v 0.0.1
