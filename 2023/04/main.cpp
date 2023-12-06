#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <deque>
#include <algorithm>
#include <cmath>

using namespace std;

void skip_char(const char* s, const char c, size_t& idx)
{
    while (s[idx] == c)
        ++idx;
}

void goto_char(const char* s, const char c, size_t& idx)
{
    while (s[idx] != '\0' && s[idx] != c)
        ++idx;
}

void parse(const string& line, vector<int>& winning_numbers, vector<int>& numbers)
{
    const char* s = line.c_str();
    size_t idx = 0;

    goto_char(s, ':', idx);
    ++idx;

    skip_char(s, ' ', idx);
    while (s[idx] != '|')
    {
        winning_numbers.push_back(atoi(&s[idx]));
        goto_char(s, ' ', idx);
        skip_char(s, ' ', idx);
    }

    ++idx;

    skip_char(s, ' ', idx);
    while (s[idx] != '\0')
    {
        numbers.push_back(atoi(&s[idx]));
        goto_char(s, ' ', idx);
        skip_char(s, ' ', idx);
    }
}

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

    int part1_points = 0;

    int part2_points = 0;
    deque<int> copies;

    int card_id = 1;

    string line;
    while (getline(input_file, line))
    {
        vector<int> winning_numbers;
        vector<int> numbers;
        parse(line, winning_numbers, numbers);

        size_t match_count = 0;
        for (const auto n : numbers)
        {
            if (find(winning_numbers.begin(), winning_numbers.end(), n) != winning_numbers.end())
            {
                ++match_count;
            }
        }

        if (match_count > 0)
        {
            part1_points += pow(2, match_count - 1);
        }

        int copy_count = 1; // 1 for the original card
        if (!copies.empty())
        {
            const int current_copies_count = copies.front();

            copy_count += current_copies_count;

            copies.pop_front();
        }

        part2_points += copy_count;

        if (copies.size() < match_count)
        {
            copies.resize(match_count, 0);
        }
        for (auto idx = 0; idx < match_count; ++idx)
        {
            copies[idx] += copy_count;
        }

        card_id += 1;
    }

    printf("Part 01 total points: %d\n", part1_points);
    printf("Part 02 total points: %d\n", part2_points);

    return 0;
}
