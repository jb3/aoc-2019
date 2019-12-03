"""Entry point for application."""
import typing

from parser import parse
from wire import Point


def get_input() -> typing.List[str]:
    """Open input file and return lines."""
    with open("../input", "r") as f:
        return f.readlines()


if __name__ == "__main__":
    wires = [parse(inst.split(",")) for inst in get_input()]

    wire1_points = set(wires[0].points)
    wire2_points = set(wires[1].points)

    intersections = list(wire1_points.intersection(wire2_points))

    intersections.remove(Point(0, 0))

    orig = Point(0, 0)
    intersections.sort(key=lambda point: orig.manhattan_distance(point))

    part1 = intersections[0].manhattan_distance(orig)

    print(f"Part 1: {part1}")

    trip_distances = map(
        lambda p: (p, wires[0].dist_to(p) + wires[1].dist_to(p)),
        intersections
    )

    trip_distances = list(trip_distances)

    trip_distances.sort(key=lambda y: y[1])

    print(f"Part 2: {trip_distances[0][1]}")
