"""Complex exception handling — edge cases from both staticfg and python-graphs."""


def multiple_excepts():
    """Multiple except handlers including bare except."""
    try:
        x = risky()
    except ValueError:
        x = 0
    except TypeError as e:
        x = str(e)
    except:
        x = None
    return x


def nested_try():
    """Nested try-except blocks."""
    try:
        try:
            x = inner_risky()
        except ValueError:
            x = 0
    except Exception:
        x = -1
    return x


def try_except_else():
    """Try with else clause (runs when no exception)."""
    try:
        x = compute()
    except ValueError:
        x = 0
    else:
        x = x + 1
    return x


def try_except_else_finally():
    """All four clauses: try/except/else/finally."""
    try:
        result = compute()
    except ValueError:
        result = 0
    else:
        result = result * 2
    finally:
        cleanup()
    return result


def break_in_try():
    """Break inside a try block within a loop."""
    for i in range(10):
        try:
            if i == 5:
                break
            risky(i)
        except Exception:
            pass
    return i


def continue_in_except():
    """Continue inside an except handler."""
    results = []
    for i in range(10):
        try:
            x = risky(i)
        except Exception:
            continue
        results.append(x)
    return results


def return_in_finally():
    """Return in finally — overrides other returns."""
    try:
        return 1
    finally:
        return 2


def raise_in_except():
    """Re-raise with different exception type."""
    try:
        risky()
    except ValueError as e:
        raise RuntimeError("wrapped") from e


def bare_raise():
    """Bare raise to re-raise current exception."""
    try:
        risky()
    except Exception:
        log_error()
        raise
