FROM rust:1.73

COPY . .

RUN cargo build --release

CMD ["./target/release/tracking-app"]

# to build docker build -t tracking-app .