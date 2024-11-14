from celery import Celery

app = Celery('tasks', broker='pyamqp://admin:password@localhost:5672//')

@app.task
def hello_world():
    print("Hello world!")
