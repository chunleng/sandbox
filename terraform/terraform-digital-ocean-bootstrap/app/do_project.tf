data "digitalocean_project" "project" {
  name = "${lower(var.stage)}-sandbox"
}
