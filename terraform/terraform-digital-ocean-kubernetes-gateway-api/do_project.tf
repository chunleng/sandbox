resource "digitalocean_project" "project" {
  name        = "${var.project}-project"
  description = "Deployment of ${var.project}"
  resources = [
    digitalocean_kubernetes_cluster.k8s.urn
  ]
}
