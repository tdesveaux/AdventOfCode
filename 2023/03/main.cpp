#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>

using namespace std;

struct Symbol
{
    int x;
    int y;
};

struct Number
{
    int value;

    // X bounds, inclusive
    int x_start;
    int x_stop;

    int y;

    bool has_contact(const Symbol& s) const
    {
        return (
            this->y >= (s.y - 1) && this->y <= (s.y + 1)
        ) && (
            s.x >= (this->x_start - 1) && s.x <= (this->x_stop + 1)
        );
    }
};

bool is_number(char c)
{
    return c >= '0' && c <= '9';
}

bool parse(const char* input_filepath, vector<Symbol> &symbols, vector<Number>& numbers)
{
    fstream input_file;

    input_file.open(input_filepath, ios::in);
    if (!input_file.is_open())
    {
        fprintf(stderr, "Failed to open file '%s'\n", input_filepath);
        return false;
    }

    int y = 0;

    string line;
    while (getline(input_file, line))
    {
        const auto line_len = line.length();
        int x = 0;
        for (int x = 0; x < line_len; ++x)
        {
            if (line[x] == '.')
                continue;

            if (is_number(line[x]))
            {
                int x_start = x;

                while (is_number(line[x + 1]))
                    ++x;

                numbers.push_back(Number{.value = atoi(&line.c_str()[x_start]), .x_start = x_start, .x_stop = x, .y = y});
            }
            else
            {
                symbols.push_back(Symbol{.x = x, .y = y});
            }

        }

        ++y;
    }

    return true;
}


int main(int argc, char** argv)
{
    if (argc != 2)
    {
        fprintf(stderr, "Usage: bin file_path\n");
        return 1;
    }
    const char* input_filepath = argv[1];

    vector<Symbol> symbols;
    vector<Number> numbers;
    if (!parse(input_filepath, symbols, numbers))
    {
        return -1;
    }

    printf("Symbols:\n");
    for (const auto& s : symbols)
    {
        printf("\t(%d, %d)\n", s.x, s.y);
    }
    printf("\n");
    printf("Numbers:\n");
    for (const auto& n : numbers)
    {
        printf("\t%d([%d, %d], %d)\n", n.value, n.x_start, n.x_stop, n.y);
    }

    int sum = 0;

    printf("\n\n");
    for (const auto& n : numbers)
    {
        for (const auto& s : symbols)
        {
            if (n.has_contact(s))
            {
                sum += n.value;
                break;
            }
        }
    }

    printf("Part 01 sum: %d\n", sum);

    return 0;
}
