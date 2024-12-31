resource "digitalocean_kubernetes_cluster" "k8s" {
  name   = "${var.project}-k8s"
  region = var.default_region

  # doctl kubernetes options versions
  version  = "1.31.1-do.5"
  vpc_uuid = digitalocean_vpc.vpc.id

  node_pool {
    name = "${var.project}-k8s-worker"
    # doctl compute size list
    size       = "s-1vcpu-2gb"
    node_count = 1
  }
}

