import ast
from functools import cmp_to_key


def checkCorrectness(l1, l2):
    if isinstance(l1, int) and isinstance(l2, int):
        if l1 < l2:
            return 1
        if l1 > l2:
            return -1
        return 0
    if isinstance(l1, list) and isinstance(l2, list):
        len1 = len(l1)
        len2 = len(l2)
        for i in range(min(len1, len2)):
            ans = checkCorrectness(l1[i], l2[i])
            if ans == 0:
                continue
            return ans
        if len1 < len2:
            return 1
        if len1 > len2:
            return -1
    if isinstance(l1, int):
        return checkCorrectness([l1], l2)
    if isinstance(l2, int):
        return checkCorrectness(l1, [l2])
    return 0


with open("input") as f:
    idx_sum = 0
    lines = [ast.literal_eval(i.strip()) for i in f.readlines() if i.strip() != ""]
    for i in range(len(lines) // 2):
        l1 = lines[2*i]
        l2 = lines[2*i+1]
        if checkCorrectness(l1, l2) == 1:
            idx_sum += i + 1
    print("part 1:", idx_sum)
    lines = lines + [[[2]]] + [[[6]]]
    sort = sorted(lines, key=cmp_to_key(checkCorrectness), reverse=True)
    print("part 2:", (sort.index([[2]]) + 1) * (sort.index([[6]]) + 1))

with open("others-solution-and-input/input") as f:
    idx_sum = 0
    lines = [ast.literal_eval(i.strip()) for i in f.readlines() if i.strip() != ""]
    for i in range(len(lines) // 2):
        l1 = lines[2*i]
        l2 = lines[2*i+1]
        if checkCorrectness(l1, l2) == 1:
            idx_sum += i + 1
    print("part 1:", idx_sum)
    lines = lines + [[[2]]] + [[[6]]]
    sort = sorted(lines, key=cmp_to_key(checkCorrectness), reverse=True)
    print("part 2:", (sort.index([[2]]) + 1) * (sort.index([[6]]) + 1))
