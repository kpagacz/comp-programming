import re
import numpy as np
from ortools.linear_solver import pywraplp

def parse_input(input: list[str]):
    def parse_line(line: str):
        # Extract the pattern in brackets (e.g., [.##.])
        pattern_match = re.search(r'\[([.#]+)\]', line)
        if pattern_match:
            pattern = pattern_match.group(1)
            pattern_array = np.array([1 if c == '#' else 0 for c in pattern])
        else:
            pattern_array = np.array([])

        # Extract lists of numbers in parentheses (e.g., (3), (1,3), etc.)
        paren_lists = re.findall(r'\(([^)]+)\)', line)
        paren_lists = [list(map(int, lst.split(','))) for lst in paren_lists]

        # Extract the list of numbers in curly brackets (e.g., {3,5,4,7})
        curly_match = re.search(r'\{([^}]+)\}', line)
        if curly_match:
            curly_list = list(map(int, curly_match.group(1).split(',')))
        else:
            curly_list = []

        return pattern_array, paren_lists, curly_list

    return list(map(parse_line, input))

def solve_machine(bulbs, buttons: list[list[int]], joltage: list[int]):
    # Solver
    solver = pywraplp.Solver.CreateSolver("SCIP")
    if not solver:
        print("SCIP not found")
        return 0

    # Variables to find
    variables = [solver.IntVar(0, solver.infinity(), f"button_{i}") for i in range(len(buttons))]

    # Constraints
    for bulb in range(len(bulbs)):
        buttons_affecting_the_bulb = []
        for button_id in range(len(buttons)):
            if bulb in buttons[button_id]:
                buttons_affecting_the_bulb.append(variables[button_id])
        solver.Add(sum(buttons_affecting_the_bulb) == joltage[bulb])

    # Objective
    solver.Minimize(sum(variables))

    # Solve
    solver.Solve()

    return sum([int(var.solution_value()) for var in variables])

def part2(input: list[str]):
    machines = parse_input(input)
    for bulbs, buttons, joltage in machines:
        solve_machine(bulbs, buttons, joltage)
    return sum([solve_machine(bulbs, buttons, joltage) for bulbs,buttons,joltage in machines])

def read_file(path: str):
    with open(path, "r") as file:
        return file.readlines()

if __name__ == "__main__":
    test = read_file("./test")
    input = read_file("./input")
    part2_test = part2(test)
    print(f"Part 2 (test): {part2_test} expected: 33")
    print(f"Part 2: {part2(input)}")
