#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

const auto seeds_prefix_len = (sizeof("seeds: ") / sizeof(char)) - 1;

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

struct Mapping
{
    long dest_range_start;
    long src_range_start;
    long range_length;

    void init_from_line(const string& line)
    {
        const auto line_len = line.length();
        const char* s = line.c_str();
        size_t idx = 0;

        this->dest_range_start = atol(&s[idx]);
        goto_char(s, ' ', idx);
        skip_char(s, ' ', idx);

        this->src_range_start = atol(&s[idx]);
        goto_char(s, ' ', idx);
        skip_char(s, ' ', idx);

        this->range_length = atol(&s[idx]);
        goto_char(s, ' ', idx);
        skip_char(s, ' ', idx);
    }

    bool map_if_in_range(long src, long& out)
    {
        if (src < src_range_start || (src - src_range_start) >= range_length)
        {
            return false;
        }

        out = dest_range_start + (src - src_range_start);

        return true;
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

    vector<long> values;

    {
        string seed_line;
        getline(input_file, seed_line);

        const char* s = seed_line.c_str();
        for (auto idx = seeds_prefix_len, line_len = seed_line.length(); idx < line_len;)
        {
            values.push_back(atol(&s[idx]));
            goto_char(s, ' ', idx);
            skip_char(s, ' ', idx);
        }

        // Skip empty
        getline(input_file, seed_line);
    }

    string line;
    // This will get the 'rule' line
    while (getline(input_file, line))
    {
        vector<long> mapped_values = values;

        for (const auto v : values)
        {
            printf("%lu, ", v);
        }
        printf("\n");

        // Stop on empty line which split each rules
        while (getline(input_file, line) && line.length() > 0)
        {
            Mapping mapping;
            mapping.init_from_line(line);

            for (auto idx = 0; idx < values.size(); ++idx)
            {
                long out;
                if (mapping.map_if_in_range(values[idx], out))
                {
                    mapped_values[idx] = out;
                    continue;
                }
            }
        }

        values.swap(mapped_values);
    }

    printf("====================\n");
    for (const auto v : values)
    {
        printf("%lu, ", v);
    }
    printf("\n");

    const auto min_values = *min_element(values.begin(), values.end());
    printf("Part 01 min value: %lu\n", min_values);

    return 0;
}
