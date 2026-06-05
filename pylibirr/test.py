import pylibirr

import asyncio


async def main() -> None:
    routes = await pylibirr.fetch_routes(152873)
    print(routes)


asyncio.run(main())
