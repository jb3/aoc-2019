"""Entry point for Advent of Code day 6."""
import typing

from planet import Planet


def get_input() -> typing.List[typing.Tuple[str, str]]:
    """Fetch input and parse from AoC format to a tuple."""
    with open("../input") as f:
        return [tuple(x.strip().split(")")) for x in f.readlines()]


def find_links(
            planet: Planet,
            links: typing.List[typing.Tuple[str, str]]
        ) -> typing.List[typing.Tuple[str, str]]:
    """Locate all planets children of argument planet."""
    return [(planet.name, x) for (y, x) in links if y == planet.name]


def add_planet(parent: Planet, name: str) -> Planet:
    """Add a new planet as a child of another planet."""
    planet = Planet(name, parent)
    parent.add_child(planet)

    return planet


def find_and_add(
            planet: Planet,
            links: typing.List[typing.Tuple[str, str]],
            planets: typing.Dict[str, Planet]
        ) -> None:
    """Find all children of the planet and register them."""
    found = find_links(planet, links)

    for link in found:
        child = add_planet(planet, link[1])
        planets[child.name] = child

        find_and_add(child, links, planets)


def find_number_of_transfers(
    a: Planet,
    b: Planet,
    others: typing.List[Planet]
) -> Planet:
    """Find the number of transfers between two celestial bodies."""
    from_a_to_com = a.route_to(planets["COM"])
    from_b_to_com = b.route_to(planets["COM"])

    passed_through = set(from_a_to_com).intersection(set(from_b_to_com))

    visited = list(passed_through)

    visited.sort(key=lambda v: len(a.route_to(v)) + len(b.route_to(v)))

    a_journey = a.route_to(visited[0])
    b_journey = b.route_to(visited[0])

    a_journey.append(visited[0])
    b_journey.reverse()

    total_journey = a_journey + b_journey

    return len(total_journey) - 1


if __name__ == "__main__":
    links = get_input()

    planets = {}

    com = Planet("COM", None)

    planets["COM"] = com

    find_and_add(com, links, planets)

    total_distance = 0
    for planet in planets.values():
        if planet.name == "COM":
            continue

        total_distance += planet.distance_to(com)

    print(f"Part 1: {total_distance}")

    transfers = find_number_of_transfers(
                    planets["YOU"],
                    planets["SAN"],
                    planets
                )

    print(f"Part 2: {transfers}")
