"""Module providing utilities for parsing input data."""
import typing

from wire import Point, Wire


def parse(instructions: typing.List[str]) -> Wire:
    """Calculate the points upon a wire and create a class from them."""
    points = [Point(0, 0)]

    for instruction in instructions:
        dir = instruction[:1]
        dist = int(instruction[1:])

        for _ in range(dist):
            if dir == "R":
                next_point = points[-1].right()
            elif dir == "L":
                next_point = points[-1].left()
            elif dir == "U":
                next_point = points[-1].up()
            else:
                next_point = points[-1].down()

            points.append(next_point)

    return Wire(points)
