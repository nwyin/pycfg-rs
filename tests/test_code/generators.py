"""Generator functions with yield/yield from."""


def simple_generator():
    """Basic generator with yield."""
    yield 1
    yield 2
    yield 3


def generator_with_loop():
    """Generator yielding in a loop."""
    for i in range(10):
        yield i * 2


def generator_with_condition():
    """Generator with conditional yield."""
    for i in range(20):
        if i % 3 == 0:
            yield i


def yield_from_example():
    """yield from delegates to sub-generator."""
    yield from range(5)
    yield from range(10, 15)


def generator_with_return():
    """Generator that has an explicit return."""
    for i in range(10):
        if i == 5:
            return
        yield i


def generator_with_send():
    """Generator that uses yield as expression (for .send())."""
    total = 0
    while True:
        value = yield total
        if value is None:
            break
        total += value
