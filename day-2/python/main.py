"""Python implementation for AoC task."""

import itertools
import typing

from interpreter import Interpreter

TO_FIND = 19690720


def get_codes() -> typing.List[int]:
    """Fetch intcode from the input file."""
    with open("../input") as f:
        codes = [int(x) for x in f.read().split(",")]

    return codes


if __name__ == "__main__":
    codes = get_codes()

    # Part 1

    # Account for error
    codes[1] = 12
    codes[2] = 2

    inter = Interpreter(codes[:])

    while not inter.halted:
        inter.step()

    print(f"Part 1: {inter.codes[0]}")

    # Part 2

    for noun, verb in itertools.product(range(0, 100), repeat=2):
        codes[1] = noun
        codes[2] = verb

        inter.codes = codes[:]
        inter.reset()

        while not inter.halted:
            inter.step()

        if inter.codes[0] == TO_FIND:
            print(f"Part 2: 100 * {noun} + {verb} = {100 * noun + verb}")
            break
