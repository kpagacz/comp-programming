#include <fstream>
#include <iostream>
#include <unordered_map>

class Solution {
 public:
  uint64_t score(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    uint64_t score = 0;
    char opponent, player;
    while (input >> opponent) {
      input >> player;
      score += roundScore(opponent, player);
    }
    return score;
  }

  uint64_t part2(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    uint64_t score = 0;
    char opponent, outcome;
    while (input >> opponent) {
      input >> outcome;
      score += (outcome - 'X') * 3;
      switch (outcome) {
        case 'X':  // need to lose
          score += (opponent - 'A' + 2) % 3 + 1;
          break;
        case 'Y':  // need to draw
          score += opponent - 'A' + 1;
          break;
        case 'Z':  // need to win
          score += (opponent - 'A' + 1) % 3 + 1;
          break;
      }
    }
    return score;
  }

 private:
  uint64_t roundScore(const char& opponent, const char& player) {
    uint64_t roundScore = player - 'X' + 1;
    if ((opponent - 'A') == (player - 'X')) roundScore += 3;
    else if ((opponent - 'A' + 1) % 3 == (player - 'X')) roundScore += 6;
    return roundScore;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.score("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
