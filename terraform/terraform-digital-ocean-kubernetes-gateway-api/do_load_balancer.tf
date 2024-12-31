resource "digitalocean_loadbalancer" "lb" {
  name     = "${var.project}-lb"
  region   = var.default_region
  vpc_uuid = digitalocean_vpc.vpc.id

  # This is just dummy setting that will be overridden by the k8s manifest later when we install ApiSix on the cluster.
  forwarding_rule {
    entry_port     = 80
    entry_protocol = "http"

    target_port     = 31448
    target_protocol = "http"
  }

  lifecycle {
    ignore_changes = [
      forwarding_rule
    ]
  }
}

