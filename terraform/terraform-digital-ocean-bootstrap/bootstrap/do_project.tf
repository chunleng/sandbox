resource "digitalocean_project" "project" {
  name        = "${lower(var.stage)}-${var.project}"
  description = "${var.stage} deployment of ${var.project}"
  environment = var.stage
  resources = [
    digitalocean_spaces_bucket.bootstrap.urn
  ]
}
