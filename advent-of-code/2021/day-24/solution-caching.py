import copy
import functools
import sys

sys.setrecursionlimit(10**7)

lines = [line.strip() for line in sys.stdin.readlines()]
ops = [line.split() for line in lines]
print(ops)

@functools.lru_cache(maxsize=None)
def search(op_index, w_value, x_value, y_value, z_value):
    if z_value > 10 ** 9:
        return (False, '')
    if op_index >= len(ops):
        return (z_value == 0, '')

    values = {'w': w_value, 'x': x_value, 'y': y_value, 'z': z_value}
    def evaluate(var):
        return values[var] if var in values else int(var)

    op = ops[op_index]
    if op[0] == 'inp':
        for d in range(1, 10):
            values[op[1]] = d
            result = search(op_index + 1, values['w'], values['x'], values['y'], values['z'])

            if result[0]:
                print(op_index, w_value, x_value, y_value, z_value, str(d) + result[1])
                return (True, str(d) + result[1])

        return (False, '')

    second = evaluate(op[2])

    if op[0] == "add":
        values[op[1]] += second
    elif op[0] == "mul":
        values[op[1]] *= second
    elif op[0] == "div":
        if second == 0:
            return (False, 0)
        values[op[1]] = int(values[op[1]] / second)
    elif op[0] == "mod":
        if values[op[1]] < 0 or second <= 0:
            return (False, 0)
        values[op[1]] %= second
    elif op[0] == "eql":
        values[op[1]] = 1 if values[op[1]] == second else 0
    else:
        assert False

    return search(op_index + 1, values['w'], values['x'], values['y'], values['z'])

print(search(0,0,0,0,0))
