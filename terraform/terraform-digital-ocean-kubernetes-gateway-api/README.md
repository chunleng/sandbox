# Terraform Digital Ocean Kubernetes Gateway API

Project to test out how to use Kubernetes Gateway API on Digital Ocean
Kubernetes cluster.

## Status

Working

## Getting Started

A basic project can be setup via the following steps:

- Obtain and set up `DIGITALOCEAN_TOKEN` environment variable through the
  [tokens page](https://cloud.digitalocean.com/account/api/tokens).

```bash
terraform init
terraform apply -var-file=base.tfvars
  # <IP> and <LB_ID> can be found here

# Create context if not exist
doctl kubernetes cluster kubeconfig save sandbox-k8s
# Change to context if exists
kubectl config use-context do-sfo3-sandbox-k8s

# Install api-gateway CRD
kubectl apply -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v1.2.1/standard-install.yaml

helm repo add apisix https://charts.apiseven.com
# https://artifacthub.io/packages/helm/apisix/apisix/2.10.0
helm install apisix apisix/apisix \
  --version 2.10.0 \
  --namespace ingress-apisix \
  --create-namespace \
  --set service.type=LoadBalancer \
  --set service.annotations.kubernetes\\.digitalocean\\.com/load-balancer-id=<LB_ID> \
  --set ingress-controller.enabled=true \
  --set ingress-controller.config.apisix.serviceNamespace=ingress-apisix \
  --set ingress-controller.config.kubernetes.enableApiGateways=true
# The load balancer name needs to be applied after the resource is linked, if
# not it will hit validation error
helm upgrade apisix apisix/apisix \
  --version 2.10.0 \
  --namespace ingress-apisix \
  --create-namespace \
  --set service.type=LoadBalancer \
  --set service.annotations.kubernetes\\.digitalocean\\.com/load-balancer-id=<LB_ID> \
  --set service.annotations.service\\.beta\\.kubernetes\\.io/do-loadbalancer-name=sandbox-lb \
  --set ingress-controller.enabled=true \
  --set ingress-controller.config.apisix.serviceNamespace=ingress-apisix \
  --set ingress-controller.config.kubernetes.enableApiGateways=true
# We need to add the following certificate setup to `service.annotations` if we
# are using https
#  --set service.annotations.service\\.beta\\.kubernetes\\.io/do-loadbalancer-certificate-id=<CERT_ID> \
#  --set service.annotations.service\\.beta\\.kubernetes\\.io/do-loadbalancer-protocol=https \
#  --set service.annotations.service\\.beta\\.kubernetes\\.io/do-loadbalancer-tls-ports=443
kubectl apply -k ./k8s/base

# With the parameters from the output of commands above, we can test if it all
# works, using the following
curl "<IP>" -H "Host: www.example.com"
```

## Thoughts

- As of writing, with ApiSix helm `v2.10.0`, ApiSix application version
  `v3.11.0` and ingress controller version of `v1.8.0`, using `HTTPRoute`
  doesn't work to sync with the gateway, however, ApiSix CRD `ApisixRoute`
  works.

## Reference

- ApiSix Official, which uses the configuration for the latest ApiSix helm
  chart:
  <https://apisix.apache.org/docs/ingress-controller/tutorials/configure-ingress-with-gateway-api/>
- A full walkthrough of the setup of ApiSix ingress:
  <https://medium.com/@martin.hodges/installing-apisix-api-gateway-on-your-kubernetes-cluster-c6be6d844f36>
