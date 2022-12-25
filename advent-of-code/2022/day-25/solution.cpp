#include <algorithm>
#include <cassert>
#include <cmath>
#include <fstream>
#include <iostream>
#include <limits>
#include <numeric>
#include <string>
#include <vector>

class Solution {
 public:
  std::string part1(const std::string& path) {
    auto snafus = parseInput(path);
    auto sum = std::accumulate(snafus.begin(), snafus.end(), 0ll,
                               [&](const auto& sum, const auto& snafu) { return sum + snafuToNum<int64_t>(snafu); });
    std::cout << "Sum:" << sum << '\n';
    return numToSnafu<int64_t>(sum);
  }

  void tests() {
    auto snafus = parseInput("test");
    for (const auto& snafu : snafus) std::cout << snafuToNum(snafu) << '\n';

    std::cout << "1121-1110-1=0 == " << snafuToNum("1121-1110-1=0") << '\n';
  }

 private:
  template <typename NumType = int>
  std::string numToSnafu(const NumType& number) {
    auto digits = [&](const auto& number) {
      // n >= log2(2*s+1)/log2(5)
      auto lowerBound = log2(2 * number + 1) / log2(5);
      return (NumType)std::ceil(lowerBound);
    };

    std::string snafu(digits(number), '0');

    NumType rest = number;
    NumType power = std::pow(5, digits(number));
    for (int i = 0; i < snafu.size(); i++) {
      // std::cout << "Snafu:" << snafu << '\n';
      // std::cout << "rest:" << rest << '\n';
      power /= 5;
      if (rest == 0) break;
      if (rest >= 2 * power) {
        snafu[i] = '2';
        rest -= 2 * power;
        continue;
      }
      if (rest <= -2 * power) {
        snafu[i] = '=';
        rest += 2 * power;
        continue;
      }
      if (rest >= power) {
        auto restIfTwo = rest - 2 * power;
        auto restIfOne = rest - power;
        if (std::abs(restIfTwo) < restIfOne) snafu[i] = '2', rest -= 2 * power;
        else snafu[i] = '1', rest -= power;
        continue;
      }
      if (rest >= 0) {
        auto restIfOne = rest - power;
        auto restIf0 = rest;
        if (std::abs(restIfOne) < restIf0) snafu[i] = '1', rest -= power;
        else snafu[i] = '0';
        continue;
      }
      if (rest <= -power) {
        if (-power - rest < rest + 2 * power) snafu[i] = '-', rest += power;
        else snafu[i] = '=', rest += 2 * power;
        continue;
      }
      if (rest < 0) {
        auto restIfMinus = rest + power;
        auto restIf0 = rest;
        if (restIfMinus < std::abs(restIf0)) snafu[i] = '-', rest += power;
        else snafu[i] = '0';
        continue;
      }
    }

    return snafu;
  }

  template <typename NumType = int>
  NumType snafuToNum(const std::string& snafu) {
    NumType power = 1;
    NumType number = 0;
    for (auto r = snafu.rbegin(); r != snafu.rend(); r++) {
      switch (*r) {
        case '2':
          number += 2 * power;
          break;
        case '1':
          number += 1 * power;
          break;
        case '-':
          number -= power;
          break;
        case '=':
          number -= 2 * power;
      }
      assert(power < std::numeric_limits<NumType>::max() / 5);
      power *= 5;
    }

    assert(number > 0);
    return number;
  }

  std::vector<std::string> parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string number;
    std::vector<std::string> snafus;
    while (input >> number) snafus.push_back(number);
    return snafus;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  // s.tests();
  return 0;
}
