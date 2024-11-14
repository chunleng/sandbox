import asyncio
from taskiq_aio_pika import AioPikaBroker

RABBITMQ_URL = "ampq://admin:password@localhost:5672/"

broker = AioPikaBroker(RABBITMQ_URL)

@broker.task
async def hello_world():
    print("hello world")

@broker.task
async def long(job_id: int):
    print(f"Started job: {job_id}")
    await asyncio.sleep(10)
    print(f"Ended job: {job_id}")
