import functools


def compare_lists(list1, list2, level=0):
    if isinstance(list1, int) and isinstance(list2, int):
        if list1 == list2:
            return None
        return list1 < list2
    elif isinstance(list1, list) and isinstance(list2, list):
        for elem1, elem2 in zip(list1, list2):
            if (res := compare_lists(elem1, elem2, level + 1)) is not None:
                return res
        return len(list1) < len(list2) if len(list1) != len(list2) else None
    elif isinstance(list1, int) and isinstance(list2, list):
        return compare_lists([list1], list2, level + 1)
    else:
        return compare_lists(list1, [list2], level + 1)


if __name__ == "__main__":
    print("Day 13:")
    with open("./inputs/input.txt", "r") as file:
        lists, pair = [], []
        for line in file.read().split("\n"):
            if line == "":
                lists.append(pair)
                pair = []
                continue
            pair.append(eval(line))
        lists.append(pair)

    sum = 0
    for i, pair in enumerate(lists):
        if compare_lists(*pair):
            sum += i + 1
    print(f"\t1) {sum}")

    to_sort = [[[2]], [[6]]]
    for pair in lists:
        to_sort.extend(pair)

    s = sorted(
        to_sort,
        key=functools.cmp_to_key(lambda l1, l2: -1 if compare_lists(l1, l2) else 1),
    )
    i1, i2 = 0, 0
    for (i, l) in enumerate(s):
        if l == [[2]]:
            i1 = i + 1
        elif l == [[6]]:
            i2 = i + 1

    print(f"\t2) {i1*i2}")
