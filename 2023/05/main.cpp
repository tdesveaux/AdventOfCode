#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

const auto seeds_prefix_len = (sizeof("seeds: ") / sizeof(char)) - 1;

long atol_to_char(const char* s, const char c, size_t& idx)
{
    long ret = 0;
    while (s[idx] != '\0' && s[idx] != c)
    {
        ret = (ret * 10) + (s[idx] - '0');
        ++idx;
    }
    return ret;
}

struct SeedRange
{
    long begin;
    long end;

    bool operator<(const SeedRange& other)
    {
        return this->begin < other.begin;
    }
};

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

        this->dest_range_start = atol_to_char(&s[idx], ' ', idx);
        ++idx;

        this->src_range_start = atol_to_char(s, ' ', idx);
        ++idx;

        this->range_length = atol_to_char(s, ' ', idx);
        ++idx;
    }

    bool map_if_in_range(long src, long& out) const
    {
        if (src < src_range_start || (src - src_range_start) >= range_length)
        {
            return false;
        }

        out = dest_range_start + (src - src_range_start);

        return true;
    }

    bool map_if_intersect(SeedRange& seed, vector<SeedRange>& remaining) const
    {
        const auto src_range_end = this->src_range_start + this->range_length - 1;

        const bool has_intersect = (
            this->src_range_start < (seed.end) &&
            seed.begin < src_range_end
        );
        if (!has_intersect)
        {
            return false;
        }

        SeedRange intersect {
            .begin = max(this->src_range_start, seed.begin),
            .end = min(src_range_end, seed.end),
        };

        if (seed.begin < intersect.begin)
        {
            remaining.push_back(SeedRange {.begin = seed.begin, .end = intersect.begin - 1});
        }
        if (seed.end > intersect.end)
        {
            remaining.push_back(SeedRange {.begin = intersect.end + 1, .end = seed.end});
        }

        // map seed to dest
        seed.begin = (intersect.begin - this->src_range_start) + this->dest_range_start;
        seed.end = (intersect.end - this->src_range_start) + this->dest_range_start;

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
            values.push_back(atol_to_char(s, ' ', idx));
            ++idx;
        }

        // Skip empty
        getline(input_file, seed_line);
    }
    // copy from 1
    vector<SeedRange> values_2;
    values_2.reserve(values.size() / 2);
    for (size_t idx = 0; idx < values.size(); idx += 2)
    {
        values_2.push_back(SeedRange {.begin = values[idx], .end = values[idx] + values[idx+1] - 1});
    }

    vector<SeedRange> mapped_values_2;
    string line;
    // This will get the 'rule' line
    while (getline(input_file, line))
    {
        vector<long> mapped_values = values;
        mapped_values_2.clear();
        mapped_values_2.reserve(values_2.size());

        vector<Mapping> mappings;

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

            for (size_t idx = 0; idx < values_2.size(); ++idx)
            {
                auto& v = values_2[idx];

                if (mapping.map_if_intersect(v, values_2))
                {
                    mapped_values_2.push_back(v);

                    values_2.at(idx) = values_2.back();
                    values_2.pop_back();
                    --idx;
                }
            }
        }

        values.swap(mapped_values);

        // no match
        mapped_values_2.reserve(mapped_values_2.size() + values_2.size());
        for (const auto& r : values_2)
        {
            mapped_values_2.push_back(move(r));
        }
        values_2.swap(mapped_values_2);
    }

    const auto min_values = *min_element(values.begin(), values.end());
    printf("Part 01 min value: %lu\n", min_values);

    const auto min_values_2 = *min_element(values_2.begin(), values_2.end());
    printf("Part 02 min value: %lu\n", min_values_2.begin);

    return 0;
}
