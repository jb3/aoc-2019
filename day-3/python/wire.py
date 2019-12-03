"""Structures relating to the wire and points on it."""
import typing
from dataclasses import dataclass


@dataclass
class Point:
    """Class representing a point in 2D space."""

    x: int
    y: int

    def right(self: "Point") -> "Point":
        """Return a new point 1 unit right of the point."""
        return Point(self.x + 1, self.y)

    def left(self: "Point") -> "Point":
        """Return a new point 1 unit left of the point."""
        return Point(self.x - 1, self.y)

    def up(self: "Point") -> "Point":
        """Return a new point 1 unit up from the point."""
        return Point(self.x, self.y + 1)

    def down(self: "Point") -> "Point":
        """Return a new point 1 unit down from the point."""
        return Point(self.x, self.y - 1)

    def __hash__(self: "Point") -> int:
        """Convert the Point to a form which Python can hash."""
        return hash((self.x, self.y))

    def manhattan_distance(self: "Point", other: "Point") -> int:
        """Calculate the manhattan distance between two points."""
        return abs(self.x - other.x) + abs(self.y - other.y)


class Wire:
    """Class reprensenting an electrical wire."""

    def __init__(self: "Wire", points: typing.List[Point]) -> None:
        """Instantiate a new wire given a list of points."""
        self.points = points

    def dist_to(self: "Wire", point: Point) -> int:
        """Calculate how far along the wire this point is."""
        return self.points.index(point)
