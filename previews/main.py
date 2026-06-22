#!/usr/bin/env python3
import sys

# This is a standard comment
def main(a: int, b: int):
    """
    This is a docstring

    Docstrings can represent a REPL session, and some syntax highlighters support that.

    >>> 1 + 2
    3

    >>> def foo():
    ...    pass
    """
    try:
        print("%d + %d =" % (a, b), a + b)
    except Exception as e:
        print(e, file=sys.stderr)
        return 1
    return 0

if __name__ == "__main__":
    sys.exit(main(1, 2))
