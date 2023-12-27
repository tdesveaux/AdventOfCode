from dataclasses import dataclass
import sys

@dataclass
class Point:
    x: int
    y: int

points = []
with open(sys.argv[1]) as fp:
    y = 0
    while line := fp.readline():
        x = 0
        for c in line.strip():
            if c == '#':
                points.append(Point(x, y))
            x += 1
        y += 1

    length = y

non_empty_lines = set(p.y for p in points)
non_empty_columns = set(p.x for p in points)

empty_lines = set(range(length)).difference(non_empty_lines)
empty_columns = set(range(length)).difference(non_empty_columns)

# move points with deduced empty spaces
for p in points:
    p.x += sum(p.x > e for e in empty_columns)
    p.y += sum(p.y > e for e in empty_lines)

y_len = max(p.y for p in points)
x_len = max(p.x for p in points)

def manhattan_length(left: Point, right: Point):
    return abs(right.x - left.x) + abs(right.y - left.y)


part_1 = sum(
    manhattan_length(left, right)
    for l_idx, left in enumerate(points)
    for right in points[l_idx+1:]
)


print(f"part 1: {part_1}")
