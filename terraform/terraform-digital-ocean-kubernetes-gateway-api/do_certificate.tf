# If https connection is needed
#
# resource "digitalocean_certificate" "cert" {
#   name = "${var.project}-cert"
#   type = "lets_encrypt"
#
#   domains = [
#     digitalocean_domain.domain.name
#   ]
# }
#
