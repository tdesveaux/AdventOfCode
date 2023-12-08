#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath>
#include <functional>

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

static const size_t JOKER_IDX = index_of_card('J');

HandType card_value(const vector<char>& cards, bool use_jokers)
{
    vector<char> cards_count(CARD_VALUES.size(), char(0));

    for (const auto card : cards)
    {
        const auto index = index_of_card(card);
        cards_count[index] += 1;
    }

    const auto joker_count = use_jokers ? cards_count.at(JOKER_IDX) : 0;
    if (use_jokers)
    {
        cards_count.erase(cards_count.begin() + JOKER_IDX);
    }
    sort(cards_count.begin(), cards_count.end(), greater<>());
    if (use_jokers)
    {
        cards_count[0] += joker_count;
    }

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
    HandType p2_type;

    Hand(const string& hand_bid):
        cards(hand_bid.begin(), hand_bid.begin() + CARDS_COUNT),
        bid(atoi(&hand_bid.c_str()[CARDS_COUNT + 1])),
        type(card_value(vector<char>(cards), false)),
        p2_type(card_value(vector<char>(cards), true))
    {
    }
};

bool hand_comparator(const Hand& lhs, const Hand& rhs, bool use_joker)
{
    auto cmp = use_joker ? int(lhs.p2_type) - int(rhs.p2_type) : int(lhs.type) - int(rhs.type);
    if (cmp == 0)
    {
        for (size_t idx = 0; idx < Hand::CARDS_COUNT; ++idx)
        {
            const auto lhs_card = lhs.cards[idx];
            const auto rhs_card = rhs.cards[idx];

            cmp = (
                (use_joker && lhs_card == 'J' ? CARD_VALUES.size(): index_of_card(lhs_card)) -
                (use_joker && rhs_card == 'J' ? CARD_VALUES.size(): index_of_card(rhs_card))
            );
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

    sort(hands.begin(), hands.end(), bind(&hand_comparator, placeholders::_1, placeholders::_2, false));
    long total_winnings = 0;
    for (size_t hand_idx = 0; hand_idx < hands.size(); ++hand_idx)
    {
        total_winnings += (hand_idx + 1) * hands[hand_idx].bid;
    }
    printf("Part1 total winnings: %d\n", total_winnings);

    sort(hands.begin(), hands.end(), bind(&hand_comparator, placeholders::_1, placeholders::_2, true));
    long part2_total_winnings = 0;
    for (size_t hand_idx = 0; hand_idx < hands.size(); ++hand_idx)
    {
        part2_total_winnings += (hand_idx + 1) * hands[hand_idx].bid;
    }
    printf("Part2 total winnings: %d\n", part2_total_winnings);

    return 0;
}
