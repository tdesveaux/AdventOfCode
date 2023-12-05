#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>

using namespace std;

const auto prefix_len = (sizeof("Game ") / sizeof(char)) - 1;

struct GameColors
{
    uint r = 0;
    uint g = 0;
    uint b = 0;

    void max_inplace(const GameColors& other)
    {
        this->r = max(this->r, other.r);
        this->g = max(this->g, other.g);
        this->b = max(this->b, other.b);
    }
};

void skip_char(const char* s, const char c, ulong& idx)
{
    while (s[idx] == c)
        ++idx;
}

void goto_char(const char* s, const char c, ulong& idx)
{
    while (s[idx] != '\0' && s[idx] != c)
        ++idx;
}

bool is_possible(const string& line, const GameColors& target)
{
    const auto line_len = line.length();
    const auto s = line.c_str();
    GameColors game_max;

    GameColors current_turn;

    for (auto idx = line.find(':', prefix_len) + 1; idx < line_len;)
    {
        skip_char(s, ' ', idx);

        const auto value = atoi(&s[idx]);

        goto_char(s, ' ', idx);

        skip_char(s, ' ', idx);

        switch (s[idx])
        {
        case 'r':
            current_turn.r += value;
            break;
        case 'g':
            current_turn.g += value;
            break;
        case 'b':
            current_turn.b += value;
            break;
        default:
            fprintf(stderr, "Unknown value encountered '%c' in string '%s'\n", s[idx], s);
            break;
        }

        while (s[idx] != '\0' && s[idx] != ',' && s[idx] != ';')
            ++idx;

        if (s[idx] == ';')
        {
            game_max.max_inplace(current_turn);
            current_turn = GameColors();
        }

        goto_char(s, ' ', idx);
    }

    game_max.max_inplace(current_turn);

    return game_max.r <= target.r && game_max.g <= target.g && game_max.b <= target.b;
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

    GameColors part_01_target = {.r = 12, .g = 13, .b = 14};

    int possible_sum = 0;

    string line;
    while (getline(input_file, line))
    {
        const auto game_id = atoi(&line.c_str()[prefix_len]);
        if (is_possible(line, part_01_target))
        {
            possible_sum += game_id;
        }
    }

    printf("Part 01 possible sum: %d\n", possible_sum);

    return 0;
}
