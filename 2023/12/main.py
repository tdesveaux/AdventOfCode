import sys

def backtrack(record_left: str, record_right: list[int]):

    # if there is more '#' left than we need to place
    # or need to place more than there is space left
    # it's a bad solution
    if sum(e == '#' for e in record_left) > sum(record_right) or len(record_left) < sum(record_right):
        # not possible
        return 0

    # nothing to place left. We have a solution
    # previous check passing means we have no '#' left
    if not record_right:
        return 1

    to_place_count = record_right[0]

    count = 0

    # can we place the group?
    if (
        # we have space to place
        len(record_left) >= to_place_count and
        # there is not a hard separator in the span we're looking at
        '.' not in record_left[:to_place_count] and
        (
            # either we fill the remaining of the space
            len(record_left) == to_place_count or
            # or there is not a marker directly following what we're placing
            record_left[to_place_count] != '#'
        )
    ):
        count += backtrack(
            # place group + space for separator
            record_left[to_place_count+1:],
            record_right[1:],
        )

    # try shift and continue
    if record_left[0] in ['.', '?']:
        count += backtrack(
            record_left[1:],
            record_right,
        )

    return count


with open(sys.argv[1]) as fp:
    sum_p1 = 0

    while line := fp.readline():
        record_left, record_right = line.split(' ', 1)
        record_right = [int(e) for e in record_right.split(',')]

        # strip sequential dots
        record_left = '.'.join(g for g in record_left.split('.') if g)

        sum_p1 += backtrack(record_left, record_right)


print(sum_p1)
