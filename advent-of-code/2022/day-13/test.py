def compare(l, r):
    if type(l) == int and type(r) == int:
        return r - l
    elif isinstance(l, int):
        return compare([l], r)
    elif isinstance(r, int):
        return compare(l, [r])
    else:
        if l and r:
            if compare(l[0], r[0]) == 0:
                return compare(l[1:], r[1:])
            else:
                return compare(l[0], r[0])
        else:
            # at least one list is empty
            return len(r) - len(l)

with open('input', "r") as f:
    pairs = f.read().split('\n\n')
    sum = 0
    for index, pair in enumerate(pairs):
        left, right = pair.split('\n')[0:2]
        if compare(eval(left), eval(right)) > 0:
            sum += index + 1

    print(sum)
