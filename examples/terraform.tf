terraform {
  backend "http" {
    address = "http://127.0.0.1:3100/states/test"
    lock_address = "http://127.0.0.1:3100/states/test"
    unlock_address = "http://127.0.0.1:3100/states/test"

    // auth variables
    username = "admin"
    password = "admin"
  }
}

data "terraform_remote_state" "test" {
  backend = "http"
  config = {
    address = "http://127.0.0.1:3100/states"
  }
}