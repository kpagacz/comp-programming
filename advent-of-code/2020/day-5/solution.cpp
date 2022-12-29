#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <string>
#include <unordered_set>

#include "utils.cc"

using NumType = int;
class Solution {
 public:
  NumType part1(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string pass;
    NumType highestId = 0;
    while (input >> pass) highestId = std::max(highestId, getIdFromPass(pass));

    return highestId;
  }

  NumType part2(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string pass;
    std::unordered_set<NumType> passes;
    while (input >> pass) passes.insert(getIdFromPass(pass));
    for (int i = 1; i < (1 << 11) - 1; i++)
      if (!passes.contains(i) && passes.contains(i - 1) && passes.contains(i + 1)) return i;
    return -1;
  }

 private:
  NumType getIdFromPass(const std::string_view& pass) {
    NumType power = 1;
    return std::transform_reduce(pass.rbegin(), pass.rend(), 0, std::plus<NumType>(), [&](const auto& digit) {
      NumType answer = 0;
      if (digit == 'R' || digit == 'B') answer = power;
      power *= 2;
      return answer;
    });
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
