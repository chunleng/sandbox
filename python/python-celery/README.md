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

# Setup schedule
celery -A src.schedules beat

# Run scripts
python request_simple.py
python request_multiple.py
python request_random_reject.py
python request_notify.py
```

## Notes

- When a new function is added and not reloaded to the worker, the client call
  will be processed as error by the worker, and it does not get re-queued. This
  need to be considered properly when deploying new functions.
- This [section](https://github.com/celery/celery/tree/v5.4.0#bundles) of GitHub
  README is useful for information about the different backend that can be used
  for celery.
- Graceful restart works!
- There are `--soft-time-limit` and `--time-limit`, the difference is that in
  `--soft-time-limit` you can write in the application to perform rollback with
  the exception `SoftTimeLimitExceeded`
