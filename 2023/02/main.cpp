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

    void min_inplace(const GameColors& other)
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

vector<GameColors> parse_game(const string& line)
{
    const auto line_len = line.length();
    const auto s = line.c_str();

    vector<GameColors> games;
    games.push_back(GameColors());

    for (auto idx = line.find(':', prefix_len) + 1; idx < line_len;)
    {
        skip_char(s, ' ', idx);

        const auto value = atoi(&s[idx]);

        goto_char(s, ' ', idx);

        skip_char(s, ' ', idx);

        switch (s[idx])
        {
        case 'r':
            games.back().r += value;
            break;
        case 'g':
            games.back().g += value;
            break;
        case 'b':
            games.back().b += value;
            break;
        default:
            fprintf(stderr, "Unknown value encountered '%c' in string '%s'\n", s[idx], s);
            break;
        }

        while (s[idx] != '\0' && s[idx] != ',' && s[idx] != ';')
            ++idx;

        if (s[idx] == ';')
        {
            games.push_back(GameColors());
        }

        goto_char(s, ' ', idx);
    }

    return games;
}

bool is_possible(const vector<GameColors> game, const GameColors& target)
{
    GameColors game_max;

    for (auto &&turn : game)
    {
        game_max.max_inplace(turn);
    }

    return game_max.r <= target.r && game_max.g <= target.g && game_max.b <= target.b;
}

int game_power(const vector<GameColors> game)
{
    GameColors game_min;

    for (auto &&turn : game)
    {
        game_min.min_inplace(turn);
    }

    return game_min.r * game_min.g * game_min.b;
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
    int power_sum = 0;

    string line;
    while (getline(input_file, line))
    {
        const auto game_id = atoi(&line.c_str()[prefix_len]);
        const auto game = parse_game(line);
        if (is_possible(game, part_01_target))
        {
            possible_sum += game_id;
        }

        power_sum += game_power(game);
    }

    printf("Part 01 possible sum: %d\n", possible_sum);
    printf("Part 02 power sum: %d\n", power_sum);

    return 0;
}
