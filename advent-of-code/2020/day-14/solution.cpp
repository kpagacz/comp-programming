#include <algorithm>
#include <bitset>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <sstream>
#include <string>
#include <unordered_map>

using NumType = uint64_t;
using Bitmask = uint64_t;
class Solution {
 public:
  NumType part1(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    Bitmask ones, zeros;
    std::unordered_map<NumType, NumType> memory;
    while (std::getline(input, line)) {
      if (line.starts_with("mask")) {
        std::tie(ones, zeros) = parseMask(line);
      } else {
        auto [address, value] = parseMem(line);
        value |= ones;
        value &= zeros;
        memory[address] = value;
      }
    }

    return std::accumulate(memory.begin(), memory.end(), (NumType)0,
                           [](const auto& total, const auto& el) { return total + el.second; });
  }

 private:
  std::pair<Bitmask, Bitmask> parseMask(const std::string& line) {
    std::stringstream ss(line);
    std::string mask;
    ss >> mask >> mask >> mask;
    Bitmask ones = 0, zeros = UINT64_MAX;
    for (std::string::size_type it{0}; it < mask.size(); it++) {
      auto bitShift = mask.size() - 1 - it;
      if (mask[it] == '1') ones |= 1ull << bitShift;
      if (mask[it] == '0') zeros &= ~(1ull << bitShift);
    }
    std::cout << std::ranges::count(mask, 'X') << '\n';
    return {ones, zeros};
  }
  std::pair<NumType, NumType> parseMem(const std::string& line) {
    NumType address, value;
    std::sscanf(line.c_str(), "mem[%llu] = %llu", &address, &value);
    return {address, value};
  }
};
int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  // std::cout << "Part 2: " << s.part2("test") << '\n';
  return 0;
}
