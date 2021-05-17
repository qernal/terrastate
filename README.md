# TerraState

HTTP Backend for Terraform [https://www.terraform.io/docs/language/settings/backends/http.html] state files using Hyper and Rust.

> This is a draft and still a WIP

## Endpoints


## TF Config

```hcl
terraform {
  backend "http" {
    address = "http://terrastate/foo"
    lock_address = "http://terrastate/foo"
    unlock_address = "http://terrastate/foo"

    // auth variables
    username = "abc"
    password = "token"
  }
}
```

## Running

Docker-compose with etcd;

```yaml
version: '2'

networks:
  app-tier:
    driver: bridge

services:
  etcd:
    image: 'bitnami/etcd:latest'
    environment:
      - ETCD_ADVERTISE_CLIENT_URLS=http://etcd:2379
      - ETCD_ROOT_PASSWORD=pass
    networks:
      - app-tier
  terrastate:
    image: 'ghcr.io/qernal/terrastate/terrastate:local'
    environment:
      - TS_DB_HOST=etcd:2379
      - TS_DB_PASS=pass
      - TS_DB_USER=root
      - TS_SERVER_ADDRESS=0.0.0.0
      - TS_SERVER_PORT=3000
      - TF_USER=admin
      - TF_PASS=admin
    ports:
      - 80:3000
    networks:
      - app-tier
```