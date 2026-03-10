"""Complex nesting — deeply nested mixed constructs."""


def if_in_loop_in_try():
    """If inside loop inside try."""
    try:
        for i in range(10):
            if i % 2 == 0:
                process(i)
            else:
                skip(i)
    except Exception:
        handle_error()
    return True


def loop_in_if_in_loop():
    """Loop inside if inside loop."""
    for i in range(5):
        if i > 2:
            for j in range(i):
                compute(i, j)
        else:
            default(i)


def try_in_loop_with_break():
    """Try inside loop with break in except."""
    for i in range(10):
        try:
            risky(i)
        except FatalError:
            break
        except RetryError:
            continue
    return i


def match_in_loop():
    """Match statement inside a loop."""
    for event in events:
        match event:
            case "click":
                handle_click()
            case "key":
                handle_key()
            case _:
                pass


def while_with_complex_body():
    """While loop with multiple branches in body."""
    x = 100
    while x > 0:
        if x > 50:
            x -= 10
        elif x > 20:
            x -= 5
        else:
            x -= 1
        if x == 42:
            break
    return x


def deeply_nested_returns():
    """Multiple return paths through nested constructs."""
    if condition_a():
        if condition_b():
            return "ab"
        else:
            for i in range(10):
                if check(i):
                    return i
    elif condition_c():
        try:
            return compute()
        except Exception:
            return None
    return "default"
