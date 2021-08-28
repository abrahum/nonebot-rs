from asyncio.tasks import gather
import nbrs_py as nbrs
import asyncio


async def main():
    await nbrs.run()


if __name__ == "__main__":
    asyncio.run(main())
