import pylibirr

import asyncio


async def main() -> None:
    client = await pylibirr.IRRClient()
    print(await client.route(152873))


asyncio.run(main())