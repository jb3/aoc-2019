"""Solution for AoC 2019 day 1."""

part1_fuel = 0
part2_fuel = 0


def calculate_fuel(mass: int) -> int:
    """Calculate required fuel given mass."""
    mass //= 3
    mass -= 2
    return mass


with open("../input") as f:
    for mass in f:
        fuel = calculate_fuel(int(mass))

        part1_fuel += fuel
        part2_fuel += fuel

        last = fuel
        while True:
            fuel_for_fuel = calculate_fuel(last)
            if fuel_for_fuel <= 0:
                break

            part2_fuel += fuel_for_fuel
            last = fuel_for_fuel

print(f"Part 1: {part1_fuel}")
print(f"Part 2: {part2_fuel}")
