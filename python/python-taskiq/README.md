# Python TaskIQ

Test out the function of python library [TaskIQ](https://github.com/taskiq-python/taskiq)

## Status

Working

## Getting Started

```bash
# Setup RabbitMQ
docker-compose up

# Startup worker
taskiq worker src.broker:broker

# Send some tasks
python request_simple.py
python request_multiple.py
```
