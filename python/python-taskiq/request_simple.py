import asyncio
from src.broker import broker, hello_world


async def main():
    await broker.startup()
    await hello_world.kiq()
    await broker.shutdown()

if __name__ == '__main__':
    asyncio.run(main())
