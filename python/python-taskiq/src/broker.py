from taskiq_aio_pika import AioPikaBroker

RABBITMQ_URL = "ampq://admin:password@localhost:5672/"

broker = AioPikaBroker(RABBITMQ_URL)

@broker.task
async def hello_world():
    print("hello world")
