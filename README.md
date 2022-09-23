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
```
