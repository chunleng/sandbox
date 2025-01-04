# Ory Kratos

This small project to explore how to use Ory Kratos for authentication.

## Status

Working

## Using This Sandbox

- Go to <http://localhost:4455/welcome> to test out public feature like login
  and logout.
- Go to <http://localhost:4456> to test out admin feature like creating a user.

## Notes

- Distroless docker does not work on SQLite, maybe a proper RDBMS can resolve
  this issue, but this is not tested

## Useful Links

### Custom `identity.schema.json`

This can be referred to in the following
[link](https://www.ory.sh/docs/kratos/manage-identities/customize-identity-schema)

### Editing `kratos.yaml`

This [link](https://www.ory.sh/docs/kratos/configuring) provides important
information on how to configure a certain function on Kratos
