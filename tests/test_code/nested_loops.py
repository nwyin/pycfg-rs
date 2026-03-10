"""Nested loop constructs — edge cases from python-graphs test suite."""


def nested_for_while():
    """For loop containing a while loop with break/continue."""
    for i in range(10):
        j = 0
        while j < i:
            if j == 3:
                break
            if j % 2 == 0:
                j += 1
                continue
            j += 1


def nested_while_while():
    """Deeply nested while loops."""
    x = 10
    while x > 0:
        y = x
        while y > 0:
            y -= 1
        x -= 1


def triple_nested():
    """Three levels of loop nesting."""
    for i in range(3):
        for j in range(3):
            for k in range(3):
                if i == j == k:
                    break


def break_outer_via_flag():
    """Break from inner loop doesn't break outer — flag pattern."""
    found = False
    for i in range(10):
        for j in range(10):
            if i * j == 42:
                found = True
                break
        if found:
            break
    return found
