FROM rust:1.37

WORKDIR /usr/src/standupbot
COPY . .

RUN cargo install --path .

CMD ["sprint_bot"]