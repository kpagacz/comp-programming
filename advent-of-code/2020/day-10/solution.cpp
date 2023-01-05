#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <unordered_set>
#include <vector>

using NumType = uint64_t;
class Solution {
 public:
  NumType part1(const std::string& path) {
    auto numbers = parseInput(path);
    std::ranges::sort(numbers);
    int oneDiffs = 0, threeDiffs = 0;
    std::vector<NumType> differences;
    std::adjacent_difference(numbers.begin(), numbers.end(), std::back_inserter(differences));

    std::for_each(differences.begin(), differences.end(), [&](const auto& diff) {
      switch (diff) {
        case 1:
          oneDiffs++;
          break;
        case 3:
          threeDiffs++;
          break;
      }
    });

    threeDiffs++;
    return oneDiffs * threeDiffs;
  }

  NumType part2(const std::string& path) {
    auto numbers = parseInput(path);
    std::ranges::sort(numbers);
    numbers.push_back(numbers.back() + 3);

    std::unordered_set<NumType> numbersSet(numbers.begin(), numbers.end());
    std::vector<NumType> arrangements(numbers.back() + 1, (NumType)0);
    arrangements[0] = 1;

    for (int i = 1; i < arrangements.size(); i++)
      if (numbersSet.contains(i))
        for (int j = 1; j <= 3 && i - j >= 0; j++) arrangements[i] += arrangements[i - j];

    return arrangements.back();
  }

 private:
  std::vector<NumType> parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    NumType num;
    std::vector<NumType> numbers;
    while (input >> num) numbers.push_back(num);
    return numbers;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
