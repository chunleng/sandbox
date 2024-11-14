# Python Celery

Queue worker for python.

## Status

WIP

## Getting Started

```bash
# Setup RabbitMQ
docker-compose up

# Setup worker
celery -A src.tasks worker

# Run scripts
python request_simple.py
python request_multiple.py
python request_random_reject.py
python request_notify.py
```
