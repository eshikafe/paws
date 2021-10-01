FROM rust:1.55.0-slim-buster as builder

RUN USER=root cargo new --bin pawslib
WORKDIR /pawslib
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev

RUN cargo build --release --bin paws-server


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata iproute2 curl jq \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3030

ENV TZ=Etc/UTC \
    APP_USER=pawsuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /pawslib/target/release/paws-server ${APP}/paws-server

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./paws-server"]
