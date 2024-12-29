# Terraform Digital Ocean Base

Project to act as a base project to create other Digital Ocean Terraform
project.

## Status

Working

## Getting Started

A basic project can be setup via the following steps:

- Obtain and set up `DIGITALOCEAN_TOKEN` environment variable through the
  [tokens page](https://cloud.digitalocean.com/account/api/tokens).
<!-- TODO This is only necessary if you need to use Spaces -->
- Obtain and set up `SPACES_ACCESS_KEY_ID` and `SPACES_SECRET_ACCESS_KEY`
  environment variable through the [Spaces access keys
  page](https://cloud.digitalocean.com/spaces/access_keys).

```bash
terraform init
terraform apply -var-file=base.tfvars
```
