#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

class Solution {
 public:
  int64_t part1(const std::string& path) {
    auto numbers = parseInput(path);
    std::vector<int> indices(numbers.size());
    std::iota(indices.begin(), indices.end(), 0);
    std::vector<bool> moved(numbers.size(), false);

    auto isWrapping = [&](const auto& source, const auto& shift) {
      return source + shift <= 0 || source + shift >= indices.size() - 1;
    };

    int counter = 0;
    auto it = indices.begin();
    while (it != indices.end()) {
      const auto ind = *it;
      const auto i = it - indices.begin();

      const auto& shift = numbers[ind];
      if (shift == 0) {
        it = std::find(indices.begin(), indices.end(), ++counter);
        continue;
      }
      int destination;
      if (isWrapping(i, shift)) {
        if (shift > 0) destination = (i + shift) % (indices.size() - 1);
        else destination = indices.size() - 1 - (-shift + indices.size() - 1 - i) % (indices.size() - 1);
      } else {
        destination = i + shift;
      }
      if (destination >= i) std::shift_left(indices.begin() + i, indices.begin() + destination + 1, 1);
      else std::shift_right(indices.begin() + destination, indices.begin() + i + 1, 1);
      indices[destination] = ind;

      it = std::find(indices.begin(), indices.end(), ++counter);
    }

    auto zero = std::find_if(indices.begin(), indices.end(), [&](const auto& ind) { return numbers[ind] == 0; }) -
                indices.begin();
    auto first = numbers[indices[(zero + 1000) % indices.size()]],
         second = numbers[indices[(zero + 2000) % indices.size()]],
         third = numbers[indices[(zero + 3000) % indices.size()]];
    return first + second + third;
  }

  int64_t part2(const std::string& path) {
    const int64_t DECRYPTION_KEY = 811589153;
    auto numbers = parseInput(path);
    for (auto& number : numbers) number *= DECRYPTION_KEY;
    std::vector<int> indices(numbers.size());
    std::iota(indices.begin(), indices.end(), 0);
    std::vector<bool> moved(numbers.size(), false);

    for (int round = 0; round < 10; round++) {
      int counter = 0;
      auto it = std::find(indices.begin(), indices.end(), counter);
      while (it != indices.end()) {
        const auto ind = *it;
        const auto i = it - indices.begin();

        const auto& shift = numbers[ind];
        if (shift == 0) {
          it = std::find(indices.begin(), indices.end(), ++counter);
          continue;
        }
        int destination;
        if (shift > 0) destination = (i + shift) % (indices.size() - 1);
        else destination = indices.size() - 1 - (-shift + indices.size() - 1 - i) % (indices.size() - 1);
        if (destination == 0) destination = indices.size() - 1;

        if (destination >= i) std::shift_left(indices.begin() + i, indices.begin() + destination + 1, 1);
        else std::shift_right(indices.begin() + destination, indices.begin() + i + 1, 1);
        indices[destination] = ind;

        it = std::find(indices.begin(), indices.end(), ++counter);
      }
    }

    auto zero = std::find_if(indices.begin(), indices.end(), [&](const auto& ind) { return numbers[ind] == 0; }) -
                indices.begin();
    auto first = numbers[indices[(zero + 1000) % indices.size()]],
         second = numbers[indices[(zero + 2000) % indices.size()]],
         third = numbers[indices[(zero + 3000) % indices.size()]];
    return first + second + third;
  }

 private:
  std::vector<int64_t> parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::vector<int64_t> numbers;
    int64_t number;
    while (input >> number) numbers.push_back(number);
    return numbers;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << "\n";
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
