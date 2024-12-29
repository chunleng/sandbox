resource "digitalocean_vpc" "vpc" {
  name   = "${var.project}-vpc"
  region = var.default_region
}
