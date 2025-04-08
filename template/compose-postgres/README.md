# Compose Postgres

Docker compose for setting up a Postgres DB.

## Status

Working

## Getting Started

To start each version's `docker-compose` file, we can use the following command:

```bash
docker-compose -f ./data/v15/docker-compose.yml up
docker-compose -f ./data/v17/docker-compose.yml up
```

### Instance with Verify CA SSL Mode

```bash
docker-compose -f ./data/v17/docker-compose.yml -f ./data/verify-ca-ssl/docker-compose.yml up
```

## Note

- Generating verify-ca key:

  ```bash
  # When prompted for Common Name (CN), it must be set to the same value
  openssl req -new -x509 -days 365 -nodes \
    -out ./data/verify-ca-ssl/certs/server.crt \
    -keyout ./data/verify-ca-ssl/certs/server.key
  openssl req -new -nodes \
    -out ./data/verify-ca-ssl/certs/client.csr \
    -keyout ./data/verify-ca-ssl/certs/client.key
  openssl x509 -req -in ./data/verify-ca-ssl/certs/client.csr -CA \
    ./data/verify-ca-ssl/certs/server.crt \
    -CAkey ./data/verify-ca-ssl/certs/server.key \
    -CAcreateserial -out ./data/verify-ca-ssl/certs/client.crt -days 365
  ```

  * <https://docs.devart.com/studio-for-postgresql/connecting-to-db/generating-ssl-certificate.html>
