from src.tasks import failure, success

if __name__ == '__main__':
    success.delay()
    failure.delay()
