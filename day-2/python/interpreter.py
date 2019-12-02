"""Class representing the interpreter."""

from enum import Enum
from typing import List


class Opcode(Enum):
    """Class for representing intcode methods, mapping opname to opcode."""

    ADD = 1
    MULTIPLY = 2
    HALT = 99


class Interpreter:
    """Class representing the interpreter for IntCode."""

    def __init__(self: "Interpreter", codes: List[int]) -> None:
        """Instantiate a new interpreter given code."""
        self.position = 0
        self.codes = codes
        self.halted = False

    def reset(self: "Interpreter") -> None:
        """Reset the interpreter."""
        self.position = 0
        self.halted = False

    def step(self: "Interpreter") -> None:
        """Step to next instruction, fetching opcode, operands and result."""
        code = Opcode(self.codes[self.position])

        if code == Opcode.HALT:
            self.halted = True
            return

        operand_1 = self.codes[self.codes[self.position + 1]]
        operand_2 = self.codes[self.codes[self.position + 2]]

        if code == Opcode.MULTIPLY:
            result = operand_1 * operand_2
        else:
            result = operand_1 + operand_2

        self.codes[self.codes[self.position + 3]] = result

        self.position += 4
