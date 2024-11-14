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

## Note

- When the worker side is yet to be refreshed with the latest function and
  client started calling a new function, the following message appears in the
  backend:

  ```text
  task "src.broker:new" is not found. Maybe you forgot to import it?
  ```

  In the next restart of the worker, the message gets processed successfully.
- `--shutdown-timeout <seconds>` is useful for gracefully shutting down the
  queue. Making sure that the queue is completed properly before quitting.
- `--max-async-tasks <n>` and `--workers <n>` are parameters useful for
  controlling the amount of tasks a worker can take

## Not Ready for Production

Seems like there's no way to cleanly deploy with TaskIQ as it does not perform
graceful restart properly

<https://github.com/taskiq-python/taskiq/issues/325>
