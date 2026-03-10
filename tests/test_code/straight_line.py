"""Straight-line code — baseline with cyclomatic complexity 1."""


def straight_line():
    """No branching at all."""
    x = 1
    y = 2
    z = x + y
    return z


def single_statement():
    """Just a return."""
    return 42


def pass_only():
    """Just pass."""
    pass


def assignments_only():
    """Multiple assignments, no control flow."""
    a = 1
    b = a + 2
    c = b * 3
    d = c - 4
    e = d / 5
