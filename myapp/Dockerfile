# Use a imagem oficial do Rust
FROM rust:latest

# Defina o diretório de trabalho no container
WORKDIR /usr/src/myapp

# Copie o código fonte para o container
COPY . .

# Compile o projeto Rust
RUN cargo build --release

# Exponha a porta que o aplicativo usará
EXPOSE 8000

# Comando para executar o aplicativo
CMD ["./target/release/myapp"]
