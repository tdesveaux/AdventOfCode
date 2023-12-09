#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <regex>
#include <algorithm>
#include <cmath>
#include <numeric>
#include <functional>

using namespace std;

struct Choice
{
    string id;

    size_t left;
    size_t right;
};

int index_of_choice(const vector<Choice>& choices, const string& id)
{
    const auto b = choices.begin();
    const auto l = choices.end();
    const auto found_it = find_if(
        b, l,
        [id](const Choice& c)
        {
            return c.id == id;
        }
    );

    return found_it != l ? found_it - b : -1;
}

size_t find_or_insert(vector<Choice>& choices, const string& id)
{
    const auto idx = index_of_choice(choices, id);
    if (idx >= 0)
    {
        return idx;
    }

    choices.push_back(Choice {.id = id});
    return choices.size() - 1;
}

size_t next_from_instruction(const char instruction, const Choice& choice)
{
    switch (instruction)
    {
    case 'L':
        return choice.left;
        break;
    case 'R':
        return choice.right;
        break;
    default:
        fprintf(stderr, "Unknown instruction '%c'\n", instruction);
        return 0;
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

    string instructions;
    getline(input_file, instructions);

    vector<Choice> choices;

    regex choice_re("^([A-Z0-9]{3}) = \\(([A-Z0-9]{3}), ([A-Z0-9]{3})\\)$");
    string line;
    getline(input_file, line); // skip empty line

    smatch match;
    while (getline(input_file, line))
    {
        if (!regex_match(line, match, choice_re))
        {
            fprintf(stderr, "Failed to match regex on line '%s'\n", line.c_str());
            return 1;
        }

        const auto choice_idx = find_or_insert(choices, match[1].str());

        const auto left = match[2].str();
        choices[choice_idx].left = find_or_insert(choices, left);

        const auto right = match[3].str();
        choices[choice_idx].right = find_or_insert(choices, right);
    }

    size_t steps = 0;
    size_t current_choice = index_of_choice(choices, "AAA");
    const size_t end = index_of_choice(choices, "ZZZ");
    for (size_t current_inst = 0; current_choice != end; current_inst = (current_inst + 1) % instructions.length())
    {
        const auto instruction = instructions[current_inst];
        const Choice& choice = choices[current_choice];

        printf("step %d Node %s, instruction %c\n", steps, choice.id.c_str(), instruction);
        current_choice = next_from_instruction(instruction, choice);

        ++steps;
    }

    printf("Part1 steps to exit: %d\n", steps);

    vector<size_t> start_nodes_step_resolve;
    for (auto idx = 0; idx < choices.size(); ++idx)
    {
        if (choices[idx].id.back() == 'A')
        {
            auto walk_idx = idx;
            for (size_t step = 0; step < SIZE_MAX; ++step)
            {
                const auto instruction = instructions[step % instructions.length()];
                if (choices[walk_idx].id.back() == 'Z')
                {
                    start_nodes_step_resolve.push_back(step);
                    break;
                }
                walk_idx = next_from_instruction(instruction, choices[walk_idx]);
            }
        }
    }

    size_t final_step = 1;
    for (const auto& s : start_nodes_step_resolve)
    {
        final_step = lcm(final_step, s);
    }

    printf("Part2 steps to exit %lu\n", final_step);

    return 0;
}
