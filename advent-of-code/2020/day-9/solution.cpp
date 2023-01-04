#include <algorithm>
#include <cassert>
#include <deque>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <vector>

using NumType = uint64_t;

class Solution {
 public:
  NumType part1(const std::string& path, const int& windowSize = 25) {
    auto numbers = parseInput(path);
    std::deque<NumType> window;
    for (auto i : std::views::iota(0, windowSize)) window.push_back(numbers[i]);

    auto isValid = [&](const auto& num) {
      for (int first = 0; first < window.size(); first++)
        for (int second = first + 1; second < window.size(); second++)
          if (window[first] + window[second] == num) return true;
      return false;
    };

    for (int i = windowSize; i < numbers.size(); i++) {
      if (!isValid(numbers[i])) return numbers[i];
      window.pop_front();
      window.push_back(numbers[i]);
    }
    return -1;
  }

  NumType part2(const std::string& path) {
    auto numbers = parseInput(path);
    const auto& goal = part1(path);

    NumType start = 0, end = 0, current = numbers[0];

    while (current != goal) {
      assert(start <= end);
      assert(end < numbers.size());
      if (current < goal) end++, current += numbers[end];
      else current -= numbers[start++];
    }
    assert(std::accumulate(numbers.begin() + start, numbers.begin() + end + 1, (NumType)0) == goal);

    return *std::min_element(numbers.begin() + start, numbers.begin() + end + 1) +
           *std::max_element(numbers.begin() + start, numbers.begin() + end + 1);
  }

 private:
  std::vector<NumType> parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::vector<NumType> numbers;
    NumType number;
    while (input >> number) numbers.push_back(number);
    return numbers;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
