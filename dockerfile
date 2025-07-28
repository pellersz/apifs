FROM rust:1.67

WORKDIR /usr/src/myapp
COPY . .

Run rustup update
RUN cargo build -r

CMD ["bash"]
