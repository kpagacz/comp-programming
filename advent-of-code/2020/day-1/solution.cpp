#include <fstream>
#include <iostream>
#include <unordered_set>
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
};
int main() {
  Solution s;
  std::cout << s.part1("input") << '\n';
}
