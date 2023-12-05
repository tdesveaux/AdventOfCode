#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>

using namespace std;

bool is_number(char c)
{
    return c >= '0' && c <= '9';
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

    ulong sum = 0;

    string line;
    while (getline(input_file, line))
    {
        const size_t line_len = line.length();
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
        sum += (line[idx] - '0') * 10;

        long rev_idx = line_len - 1;
        while (rev_idx >= idx && !is_number(line[rev_idx]))
        {
            --rev_idx;
        }
        sum += (line[rev_idx] - '0');
    }

    printf("Found sum of calibration: %lu\n", sum);

    return 0;
}
