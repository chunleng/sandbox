from src.tasks import long

if __name__ == '__main__':
    for i in range(10):
        long.delay(i)
