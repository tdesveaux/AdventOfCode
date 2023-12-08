#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <regex>

using namespace std;

long count_success_possibilities(long time, long distance_record)
{
    for (long time_spent = 0; time_spent <= time; ++time_spent)
    {
        const auto distance = time_spent * (time - time_spent);
        // printf("\t %ld -> %ld (vs %ld)\n", time_spent, distance, distance_record);
        if (distance > distance_record)
        {
            return (time - time_spent + 1) - time_spent;
        }
    }

    return 0;
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

    string times;
    getline(input_file, times);

    string distances;
    getline(input_file, distances);

    std::regex num_regex("(\\d+)");

    auto times_it = std::sregex_iterator(times.begin(), times.end(), num_regex);
    auto distances_it = std::sregex_iterator(distances.begin(), distances.end(), num_regex);

    int part1_prod = 1;

    string part2_time_s;
    string part2_distance_record_s;

    const auto end_it = std::sregex_iterator();
    for (; times_it != end_it && distances_it != end_it; ++times_it, ++distances_it)
    {
        const auto time_s = (*times_it).str();
        const auto distance_record_s = (*distances_it).str();

        part2_time_s += time_s;
        part2_distance_record_s += distance_record_s;

        const auto time = stoi(time_s);
        const auto distance_record = stoi(distance_record_s);

        printf("%d -> %d\n", time, distance_record);

        int min_time = (distance_record / time);

        int race_success_ways_count = count_success_possibilities(time, distance_record);
        if (race_success_ways_count > 0)
        {
            part1_prod *= race_success_ways_count;
        }
    }

    printf("Part 01, ways to beat record: %d\n", part1_prod);

    const auto part2_time = stol(part2_time_s);
    const auto part2_distance_record = stol(part2_distance_record_s);

    int part2_success_ways_count = count_success_possibilities(part2_time, part2_distance_record);
    printf("Part 02, ways to beat record: %ld\n", part2_success_ways_count);

    return 0;
}
