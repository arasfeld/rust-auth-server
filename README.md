# rust-auth-server

## Getting started

### Install

```bash
# run docker containers in the background
$ docker-compose up -d

# install sqlx-cli for offline migrations
$ cargo install sqlx-cli --features postgres

# run migrations
$ sqlx migrate run

# generate self-signed certificate for TLS/SSL
$ openssl req -newkey rsa:2048 \
              -x509 \
              -sha256 \
              -days 3650 \
              -nodes \
              -out cert.pem \
              -keyout key.pem
```
