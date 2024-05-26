FROM cargo:0.1.0

WORKDIR /weathercli

COPY . .

RUN cargo install
RUN cargo new
RUN cargo run 

CMD ["cargo", "cargo run"]
