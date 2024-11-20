import datetime
from celery import Celery, Task
from celery.exceptions import SoftTimeLimitExceeded, Reject
from time import sleep
import random

app = Celery(broker='pyamqp://admin:password@localhost:5672//')

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

@app.task
def check_job(job_id: str):
    print(f"{job_id=} started at {datetime.datetime.now()}")


class NotifyTask(Task):
    def on_success(self, retval, task_id, args, kwargs):
        print(f"Success: {retval=} {task_id=} {args=} {kwargs=}")
    def on_failure(self, exc, task_id, args, kwargs, einfo):
        print(f"Fail: {exc=} {task_id=} {args=} {kwargs=}, {einfo}")

@app.task(base=NotifyTask)
def success() -> str:
    return "Yay!"

@app.task(base=NotifyTask)
def failure():
    raise Exception("Boo!")
