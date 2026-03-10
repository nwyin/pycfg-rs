"""Async constructs — should be flattened to synchronous CFG."""


async def async_simple():
    """Basic async function with await."""
    result = await fetch_data()
    return result


async def async_for_loop():
    """Async for loop."""
    results = []
    async for item in aiter:
        results.append(item)
    return results


async def async_with_statement():
    """Async with (context manager)."""
    async with aopen("file.txt") as f:
        data = await f.read()
    return data


async def async_complex():
    """Mixed async constructs."""
    async with session() as s:
        async for page in s.pages():
            data = await page.content()
            if data is None:
                continue
            yield data
