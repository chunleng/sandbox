import datetime
from celery import Celery
from src.tasks import app, check_job

@app.on_after_configure.connect
def setup_periodic_tasks(sender: Celery, **_):
    for job_id in ("job_a", "job_b"):
        sender.add_periodic_task(5, check_job.s(job_id))
