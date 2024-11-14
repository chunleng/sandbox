from celery import Celery, Task
from celery.exceptions import SoftTimeLimitExceeded, Reject
from time import sleep
import random

app = Celery('tasks', broker='pyamqp://admin:password@localhost:5672//')

@app.task
def hello_world():
    print("Hello world!")

@app.task(soft_time_limit=5)
def long(job_id: int):
    print(f"Job started: {job_id}")
    try:
        # This will not sleep more than 5 seconds because of the `soft_time_limit`
        sleep(10)
    except SoftTimeLimitExceeded:
        print("Cleanup")
    print(f"Job ended: {job_id}")

@app.task(autoretry_for=(Reject,), max_retries=None, retry_backoff=True, retry_jitter=True)
def reject_random():
    i = random.randint(0, 10)
    print(f"random_number generated: {i}")
    if i < 8:
        raise Reject()
