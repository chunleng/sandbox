# Terraform Digital Ocean Bootstrap

This is a sandbox project used to start a Terraform project in digital ocean. It
consists of the bootstrap folder, which allows you to start the project and
Digital Ocean spaces necessary to get Terraform working. The `app` folder will
then use the spaces bucket set up in bootstrap as the backend to record the
states of the infrastructure.

## Status

Working

## Getting Started

A basic project can be setup via the following steps:

- Obtain and set up `DIGITALOCEAN_TOKEN` environment variable through the
  [tokens page](https://cloud.digitalocean.com/account/api/tokens).
- Obtain and set up `SPACES_ACCESS_KEY_ID` and `SPACES_SECRET_ACCESS_KEY`
  environment variable through the [Spaces access keys
  page](https://cloud.digitalocean.com/spaces/access_keys).

```bash
cd bootstrap
terraform init
terraform apply -var-file=prod.tfvars
terraform output > ../app/backend_prod.tfvars


cd ../app
terraform init -backend-config=backend_prod.tfvars
terraform apply -var-file=prod.tfvars
```

## Reference

- How to configure Digital Ocean backend:
  <https://docs.digitalocean.com/products/spaces/reference/terraform-backend/>
