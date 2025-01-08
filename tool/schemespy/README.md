# SchemaSpy

Tool to inspect a target database.

## Status

Working

## Getting Started

```bash
# To find out DB_TYPE to use
docker-compose run -it --rm schemaspy -dbHelp

# Setup credential
cp .env.example.<DB_TYPE> .env
    # Please open the file and update the credential

docker-compose up

# To view
open output/index.html
```
