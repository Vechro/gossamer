# Gossamer

![](https://raw.githubusercontent.com/Vechro/gossamer/main/static/gossamer.png)

A very simple link shortening service. Built on [Actix Web](https://actix.rs/) with [RocksDB](https://rocksdb.org/).

**See it in action on [go.vech.ro](https://go.vech.ro)**

## Setting up your own instance

This project is built with the intention that you'll serve it through Docker container, behind a web server of choice (Nginx, Apache, Caddy).

You will need a `.env` file (or some other method to set environment variables) and provide the following values:

```sh
# The server doesn't know its own domain name, so you'll have to provide one
VANITY_HOST = go.vech.ro
# These two are optional, this will default to 0.0.0.0
HOST = 127.0.0.1
# This will default to 80
PORT = 8080
# The salt will be used to randomize the shortened URLs
SALT = hello
# Where RocksDB will store everything
DATABASE_PATH = D:\Development\gossamer\data
```