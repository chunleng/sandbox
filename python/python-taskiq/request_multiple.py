import asyncio
from src.broker import broker, long


async def main():
    await broker.startup()
    for i in range(10):
        await long.kiq(i)
    await broker.shutdown()

if __name__ == '__main__':
    asyncio.run(main())
