#include <algorithm>
#include <fstream>
#include <iostream>
#include <ranges>
#include <string>
#include <unordered_map>

#include "utils.cc"

using NumType = int;
class Solution {
 public:
  NumType part1(const std::string& path, const int& ROUNDS = 2020) const {
    auto numbers = parseInput(path);
    for (auto num : numbers) std::cout << num << " ";
    std::cout << '\n';

    std::unordered_map<NumType, NumType> memory;
    for (auto round : std::views::iota(1u, numbers.size())) memory[numbers[round - 1]] = round;

    auto lastNumberSpoken = numbers.back();

    auto currentRound = numbers.size();
    while (++currentRound <= ROUNDS) {
      auto oldNumberSpoken = lastNumberSpoken;
      if (memory.contains(lastNumberSpoken)) lastNumberSpoken = currentRound - memory.at(lastNumberSpoken) - 1;
      else lastNumberSpoken = 0;
      memory[oldNumberSpoken] = currentRound - 1;
    }

    return lastNumberSpoken;
  }

 private:
  std::vector<NumType> parseInput(const std::string& path) const {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    input >> line;
    auto numbersAsStrings = utils::split(line, ",");
    std::vector<NumType> numbers;
    std::ranges::for_each(numbersAsStrings, [&](const auto& number) { numbers.push_back(std::stoull(number)); });
    return numbers;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part1("input", 3e7) << '\n';
  return 0;
}
