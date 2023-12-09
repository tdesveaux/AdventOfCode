#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <regex>
#include <algorithm>
#include <cmath>
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

    regex choice_re("^([A-Z]{3}) = \\(([A-Z]{3}), ([A-Z]{3})\\)$");
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
        switch (instruction)
        {
        case 'L':
            current_choice = choice.left;
            break;
        case 'R':
            current_choice = choice.right;
            break;
        default:
            fprintf(stderr, "Unknown instruction at %d '%c'\n", current_inst, instructions[current_inst]);
            return 1;
        }

        ++steps;
    }

    printf("Part1 steps to exit: %d\n", steps);

    return 0;
}
