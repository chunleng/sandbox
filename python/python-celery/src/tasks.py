from celery import Celery
from celery.exceptions import SoftTimeLimitExceeded
from time import sleep

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
