#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath>

using namespace std;

enum class HandType : int
{
    FiveOfAKind = 0,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
};

static const vector<char> CARD_VALUES = {'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'};

// lower is better
int index_of_card(char c)
{
    return find(CARD_VALUES.begin(), CARD_VALUES.end(), c) - CARD_VALUES.begin();
}

HandType card_value(const vector<char>& cards)
{
    vector<char> cards_count(CARD_VALUES.size(), char(0));

    for (const auto card : cards)
    {
        const auto index = index_of_card(card);
        cards_count[index] += 1;
    }

    sort(cards_count.begin(), cards_count.end(), greater<>());

    switch (cards_count.front())
    {
    case 5:
        return HandType::FiveOfAKind;
    case 4:
        return HandType::FourOfAKind;
    case 3:
        if (cards_count.at(1) == 2)
        {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    case 2:
        if (cards_count.at(1) == 2)
        {
            return HandType::TwoPair;
        }
        return HandType::OnePair;
    default:
        return HandType::HighCard;
    }
}

struct Hand
{
    static const size_t CARDS_COUNT = 5;
    vector<char> cards;
    int bid;
    HandType type;

    Hand(const string& hand_bid):
        cards(hand_bid.begin(), hand_bid.begin() + CARDS_COUNT),
        bid(atoi(&hand_bid.c_str()[CARDS_COUNT + 1])),
        type(card_value(vector<char>(cards)))
    {
    }
};

bool hand_comparator(const Hand& lhs, const Hand& rhs)
{
    auto cmp = int(lhs.type) - int(rhs.type);
    if (cmp == 0)
    {
        for (size_t idx = 0; idx < Hand::CARDS_COUNT; ++idx)
        {
            cmp = index_of_card(lhs.cards[idx]) - index_of_card(rhs.cards[idx]);
            if (cmp != 0)
            {
                break;
            }
        }
    }
    return cmp > 0;
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

    vector<Hand> hands;

    string line;
    while (getline(input_file, line))
    {
        hands.push_back(Hand(line));
    }

    sort(hands.begin(), hands.end(), hand_comparator);
    long total_winnings = 0;
    for (size_t hand_idx = 0; hand_idx < hands.size(); ++hand_idx)
    {
        total_winnings += (hand_idx + 1) * hands[hand_idx].bid;
    }
    printf("Part1 total winnings: %d\n", total_winnings);

    return 0;
}
