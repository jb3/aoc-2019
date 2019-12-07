"""Utilities for managing celestial bodies."""
import typing


class Planet:
    """Class representing a celestial body."""

    def __init__(
            self: "Planet",
            name: str,
            parent: typing.Optional["Planet"]
    ) -> None:
        """Construct a new celestial body."""
        self.name = name
        self.parent = parent
        self.children = []

    def __repr__(self: "Planet") -> str:
        """Return string representation of the planet."""
        return f"<Planet name='{self.name}' parent='{self.parent.name}'>"

    def add_child(self: "Planet", child: "Planet") -> None:
        """Register another celestial body as a direct child of this one."""
        self.children.append(child)

    def distance_to(self: "Planet", to: "Planet") -> int:
        """Find the distance to another planet."""
        distance = 0
        next_planet = self
        while True:
            if next_planet.name != to.name:
                distance += 1
                next_planet = next_planet.parent
            else:
                return distance

    def route_to(
        self: "Planet",
        to: "Planet"
    ) -> typing.Optional[typing.List["Planet"]]:
        """Find the inter-planetary route between this body and another."""
        route = []
        currently = self.parent
        while currently.name != to.name:
            route.append(currently)
            currently = currently.parent

            if currently is None:
                return None

        return route
