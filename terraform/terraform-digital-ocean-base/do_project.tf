resource "digitalocean_project" "project" {
  name        = "${var.project}-project"
  description = "Deployment of ${var.project}"
  resources = [
    # TODO add resources here to more easily manage them
  ]
}
