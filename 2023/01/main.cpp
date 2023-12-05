#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>

using namespace std;

bool is_number(char c)
{
    return c >= '0' && c <= '9';
}

ulong part_01_value(const string& line)
{
    const size_t line_len = line.length();

    ulong value = 0;

    long idx = 0;
    while (idx < line_len && !is_number(line[idx]))
    {
        ++idx;
    }
    if (idx == line_len)
    {
        fprintf(stderr, "Could not find a number on line '%s'\n", line.c_str());
        return 1;
    }
    value += (line[idx] - '0') * 10;

    long rev_idx = line_len - 1;
    while (rev_idx >= idx && !is_number(line[rev_idx]))
    {
        --rev_idx;
    }
    value += (line[rev_idx] - '0');

    return value;
}

const vector<string> numbers = {
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
};

char get_number_at(const string& line, size_t start_pos)
{
    if (is_number(line[start_pos]))
    {
        return line[start_pos] - '0';
    }

    for (size_t n = 0; n < numbers.size(); ++n)
    {
        const string& number = numbers[n];
        if (line.compare(start_pos, number.length(), number) == 0)
        {
            return n + 1;
        }
    }

    return -1;
}

long part_02_value(const string& line)
{
    const size_t line_len = line.length();

    long value = 0;

    long idx = 0;
    while (idx < line_len)
    {
        const char n = get_number_at(line, idx);
        if (n >= 0)
        {
            value += n * 10;
            break;
        }
        ++idx;
    }
    if (idx == line_len)
    {
        fprintf(stderr, "Could not find a number on line '%s'\n", line.c_str());
        return -1;
    }
    long rev_idx = line_len - 1;
    while (rev_idx >= idx)
    {
        const char n = get_number_at(line, rev_idx);
        if (n >= 0)
        {
            value += n;
            break;
        }
        --rev_idx;
    }

    return value;
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

    ulong sum_part_01 = 0;
    ulong sum_part_02 = 0;

    string line;
    while (getline(input_file, line))
    {
        sum_part_01 += part_01_value(line);
        sum_part_02 += part_02_value(line);
    }

    printf("Found Part01 sum of calibration: %lu\n", sum_part_01);
    printf("Found Part02 sum of calibration: %lu\n", sum_part_02);

    return 0;
}
