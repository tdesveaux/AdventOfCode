#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include <optional>
#include <functional>
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

    int points = 0;

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
            points += pow(2, match_count - 1);
        }
    }

    printf("Part 01 total points: %d\n", points);

    return 0;
}
