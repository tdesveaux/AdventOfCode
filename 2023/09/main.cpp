#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath>
#include <functional>

using namespace std;

long atol_to_char(const char* s, const char c, size_t& idx)
{
    long ret = 0;
    bool is_neg = false;
    if (s[idx] == '-')
    {
        ++idx;
        is_neg = true;
    }
    while (s[idx] != '\0' && s[idx] != c)
    {
        ret = (ret * 10) + (s[idx] - '0');
        ++idx;
    }
    if (is_neg)
    {
        ret *= -1;
    }
    return ret;
}

long get_next(const vector<long>& sequence)
{
    if (ranges::find_if(sequence, [](long l) {return l != 0;}) == sequence.end())
    {
        return 0;
    }

    vector<long> difference;
    difference.reserve(sequence.size() - 1);
    for (size_t idx = 1; idx < sequence.size(); ++idx)
    {
        difference.push_back(sequence[idx] - sequence[idx - 1]);
    }
    const auto value = get_next(difference);

    return sequence.back() + value;
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

    long part1_sum = 0;
    string line;
    while (getline(input_file, line))
    {
        const auto s_len = line.length();
        const char* s = line.c_str();
        vector<long> sequence;
        for (size_t idx = 0; idx < s_len; ++idx)
        {
            sequence.push_back(atol_to_char(s, ' ', idx));
        }

        const auto value = get_next(sequence);
        part1_sum += value;
    }

    printf("Part1 sum %ld\n", part1_sum);

    return 0;
}
