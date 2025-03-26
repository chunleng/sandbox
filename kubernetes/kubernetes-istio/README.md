# Kubernetes Istio

A project to test out how to set up Istio and how it works.

## Status

Working

## Feature

- Istio gateway
- SPIRE, for access control

## Getting Started

```bash
# Installing CRD for istioctl into the cluster
istioctl install -f ./k8s/istio/install.yaml -y

# Installing k8s gateway CRD
kubectl kustomize "github.com/kubernetes-sigs/gateway-api/config/crd?ref=v1.2.1" | kubectl apply -f -

# Install SPIRE
helm repo add spire https://spiffe.github.io/helm-charts-hardened/
helm install -n spire-server spire-crds spire/spire-crds --create-namespace
helm install -n spire-server spire spire/spire --wait --set global.spire.trustDomain="example.org"

# Start up the service
skaffold dev

# go to http://localhost:8080 to see the deployed app, viewed through Istio
# Gateway (./k8s/base/gateway.yaml)

# This is okay
kubectl exec -it deployment/web-internal -- wget web-internal-2 -O -

# Denied by ./k8s/base/istio/access-control.yaml
kubectl exec -it deployment/web-external -- wget web-internal -O -
```

## Useful Functions

```bash
# Show all the nodes in the service mesh, with information like their SPIFFE ID
kubectl exec -t spire-server-0 -n spire-server -c spire-server -- ./bin/spire-server entry show

# View Istio Log
# This is supported by ./k8s/base/istio/telemetry.yaml
kubectl logs deployment/web-internal -c istio-proxy -f
```

## References

- Official Istio Getting Started:
  <https://istio.io/latest/docs/setup/getting-started/>
- Official Istio Access Log:
  <https://istio.io/latest/docs/tasks/observability/logs/access-log/>
- Official Istio Authorization Policy:
  <https://istio.io/latest/docs/reference/config/security/authorization-policy/>
