output "load_balancer_ip" {
  value = digitalocean_loadbalancer.lb.ip
}

output "load_balancer_id" {
  value = digitalocean_loadbalancer.lb.id
}
