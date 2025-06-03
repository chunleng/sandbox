#!/usr/bin/env bash

set -eu

mc alias set minio http://minio:9000 admin password
mc mb --ignore-existing minio/dummy-bucket
