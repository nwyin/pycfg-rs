"""Loop else clauses — tested by python-graphs but not in our suite."""


def for_else_no_break():
    """for-else where else always runs."""
    for i in range(10):
        pass
    else:
        result = "completed"
    return result


def for_else_with_break():
    """for-else where break skips else."""
    for i in range(10):
        if i == 5:
            break
    else:
        result = "completed"
    return result


def while_else():
    """while-else clause."""
    x = 10
    while x > 0:
        x -= 1
    else:
        result = "done"
    return result


def while_else_with_break():
    """while-else where break skips else."""
    x = 10
    while x > 0:
        if x == 5:
            break
        x -= 1
    else:
        result = "exhausted"
    return result
