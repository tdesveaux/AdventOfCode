#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath>
#include <functional>

using namespace std;

bool connect_north(const char c)
{
    return (c == 'S' || c == '|' || c == 'L' || c == 'J');
}

bool connect_south(const char c)
{
    return (c == 'S' || c == '|' || c == '7' || c == 'F');
}

bool connect_west(const char c)
{
    return (c == 'S' || c == '-' || c == '7' || c == 'J');
}

bool connect_east(const char c)
{
    return (c == 'S' || c == '-' || c == 'L' || c == 'F');
}

struct Pos
{
    static const size_t default_value = numeric_limits<size_t>::max();
    size_t x = default_value;
    size_t y = default_value;

    size_t pos(size_t line_len) const
    {
        return (y * line_len) + x;
    }

    vector<Pos> get_accessible_neighbours(const vector<string>& pipes, size_t lines_len) const
    {
        vector<Pos> ret;

        const char symbol = pipes[y][x];

        // Access north?
        if (y > 0 && connect_north(symbol))
        {
            if (connect_south(pipes[y - 1][x]))
            {
                ret.push_back({.x = x, .y = y - 1});
            }
        }
        // Access south?
        if (y < lines_len && connect_south(symbol))
        {
            if (connect_north(pipes[y + 1][x]))
            {
                ret.push_back({.x = x, .y = y + 1});
            }
        }

        // Access west?
        if (x > 0 && connect_west(symbol))
        {
            if (connect_east(pipes[y][x - 1]))
            {
                ret.push_back({.x = x - 1, .y = y});
            }
        }
        // Access east?
        if (x < lines_len && connect_east(symbol))
        {
            if (connect_west(pipes[y][x + 1]))
            {
                ret.push_back({.x = x + 1, .y = y});
            }
        }

        return ret;
    }
};

int main(int argc, char** argv)
{
    if (argc != 2)
    {
        fprintf(stderr, "Usage: bin file_path\n");
        return 1;
    }
    const char* input_filepath = argv[1];

    fstream input_file;

    input_file.open(input_filepath, ios::in);
    if (!input_file.is_open())
    {
        fprintf(stderr, "Failed to open file '%s'\n", input_filepath);
        return 1;
    }

    vector<string> pipes;

    size_t line_len;
    Pos start_pos;
    string line;
    while (getline(input_file, line))
    {
        if (pipes.empty())
        {
            line_len = line.length();
            // input is a square
            pipes.reserve(line_len);
        }

        pipes.push_back(line);

        if (start_pos.x == Pos::default_value && start_pos.y == Pos::default_value)
        {
            const auto pos_in_line = line.find('S');
            if (pos_in_line != string::npos)
            {
                start_pos.x = pos_in_line;
                start_pos.y = pipes.size() - 1;
            }
        }
    }

    const auto default_weight = numeric_limits<ulong>::max();
    vector<ulong> weights(pipes.size() * line_len, default_weight);

    const auto start_neighbours = start_pos.get_accessible_neighbours(pipes, line_len);
    if (start_neighbours.size() != 2)
    {
        fprintf(stderr, "Part2 logic assume start point can be replaced by another symbol. Start symbol has %d valid neighbours\n", start_neighbours.size());
        return 1;
    }
    vector<Pos> to_walk = start_neighbours;

    // init start pos
    const auto start_flatp = start_pos.pos(line_len);
    weights[start_flatp] = 0;

    while (to_walk.size() > 0)
    {
        const Pos current = to_walk.back();
        to_walk.pop_back();

        const auto neightbours = current.get_accessible_neighbours(pipes, line_len);

        const auto min_neightbour_weight = ranges::min(neightbours, ranges::less(), [line_len, &weights](const Pos& p) { return weights[p.pos(line_len)] ; });
        auto& current_weight = weights[current.pos(line_len)];
        current_weight = min(current_weight, weights[min_neightbour_weight.pos(line_len)] + 1);

        for (const auto& n : neightbours)
        {
            const auto n_weight = weights[n.pos(line_len)];
            if (n_weight > current_weight + 1)
            {
                to_walk.push_back(n);
            }
        }
    }

    auto max_distance = 0;
    for (const auto& w : weights)
    {
        if (w != default_weight && w > max_distance)
        {
            max_distance = w;
        }
    }
    printf("Part1 max_distance = %lu\n", max_distance);

    // Since it's a loop, going in a straight line, each time we cross a pipe block, inside status changes
    size_t in_count = 0;
    for (size_t y = 0; y < pipes.size(); ++y)
    {
        const auto& line = pipes[y];
        char last_pipe = 0;
        bool is_inside = false; // start outside

        for (size_t x = 0; x < line.length(); ++x)
        {
            const auto& c = line[x];
            const bool is_in_loop = weights[y * line_len + x] != default_weight;
            if (c == '.' || !is_in_loop)
            {
                if (is_inside)
                {
                    in_count++;
                }
            }
            else
            {
                // pipe in loop
                // check if we need to change inside state

                // - are never the first pipe we encounter, and they don't change the state
                if (c == '-')
                {
                    continue;
                }

                if (c == '|')
                {
                    last_pipe = 0;
                    is_inside = !is_inside;
                }
                else if (connect_west(c) && connect_east(last_pipe))
                {
                    // We could check only north, but we need to handle 'S' which connect to everything
                    bool is_uturn = connect_north(c) == connect_north(last_pipe) || connect_south(c) == connect_south(last_pipe);
                    if (!is_uturn)
                    {
                        is_inside = !is_inside;
                    }

                    last_pipe = 0;
                }
                else
                {
                    last_pipe = c;
                }
            }
        }
    }

    printf("Part2 ground inside loop = %lu\n", in_count);

    return 0;
}
