output "endpoints" {
  value = {
    "s3" : "https://${digitalocean_spaces_bucket.bootstrap.region}.digitaloceanspaces.com"
  }
}

output "bucket" {
  value = digitalocean_spaces_bucket.bootstrap.name
}

output "key" {
  value = "${lower(var.stage)}_${var.project}.tfstate"
}

output "region" {
  value = digitalocean_spaces_bucket.bootstrap.region
}
