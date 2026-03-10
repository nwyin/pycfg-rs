"""Multiple return paths — tests return edge handling."""


def guard_clauses(x, y, z):
    """Multiple early returns (guard clause pattern)."""
    if x is None:
        return -1
    if y < 0:
        return -2
    if z == 0:
        return -3
    return x + y + z


def return_in_branches(x):
    """Every branch returns."""
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"


def return_in_loop(items):
    """Return from inside a loop."""
    for item in items:
        if item.is_valid():
            return item
    return None


def return_in_nested_if(a, b, c):
    """Returns scattered through nested ifs."""
    if a:
        if b:
            return 1
        if c:
            return 2
        return 3
    return 4
