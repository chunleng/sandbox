resource "digitalocean_spaces_bucket" "bootstrap" {
  name          = "${lower(var.stage)}-${var.project}-tf"
  region        = data.digitalocean_region.default_region.slug
  force_destroy = local.force_destroy
}
