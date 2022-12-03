#include <fstream>
#include <iostream>
#include <unordered_set>
#include <vector>
class Solution {
 public:
  uint64_t part1(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::unordered_set<uint64_t> numbers;
    uint64_t number;
    while (input >> number)
      if (numbers.count(2020 - number)) return number * (2020 - number);
      else numbers.insert(number);

    return 0;
  }
  uint64_t part2(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::vector<uint64_t> numbers;
    uint64_t number;
    while (input >> number) numbers.push_back(number);

    for (int i = 0; i < numbers.size(); i++)
      for (int j = i + 1; j < numbers.size(); j++)
        for (int k = j + 1; k < numbers.size(); k++)
          if (numbers[i] + numbers[j] + numbers[k] == 2020) return numbers[i] * numbers[j] * numbers[k];
    return 0;
  }
};

int main() {
  Solution s;
  std::cout << s.part1("input") << '\n';
  std::cout << s.part2("input") << '\n';
}
