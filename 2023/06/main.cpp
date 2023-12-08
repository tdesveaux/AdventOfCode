#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <regex>

using namespace std;

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

    string times;
    getline(input_file, times);

    string distances;
    getline(input_file, distances);

    std::regex num_regex("(\\d+)");

    auto times_it = std::sregex_iterator(times.begin(), times.end(), num_regex);
    auto distances_it = std::sregex_iterator(distances.begin(), distances.end(), num_regex);

    int part1_prod = 1;

    const auto end_it = std::sregex_iterator();
    for (; times_it != end_it && distances_it != end_it; ++times_it, ++distances_it)
    {
        const auto time = stoi((*times_it).str());
        const auto distance_record = stoi((*distances_it).str());

        printf("%d -> %d\n", time, distance_record);

        int min_time = (distance_record / time);

        int race_success_ways_count = 0;

        for (int time_spent = 0; time_spent <= time; ++time_spent)
        {
            const auto distance = time_spent * (time - time_spent);
            printf("\t %d -> %d (vs %d)\n", time_spent, distance, distance_record);
            if (distance > distance_record)
            {
                race_success_ways_count = (time - time_spent + 1) - time_spent;
                break;
            }
        }

        if (race_success_ways_count > 0)
        {
            part1_prod *= race_success_ways_count;
        }
    }

    printf("Part 01, ways to beat record: %d\n", part1_prod);

    return 0;
}
