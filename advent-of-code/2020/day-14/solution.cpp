#include <algorithm>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

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
  NumType part2(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    std::vector<Bitmask> masks;
    std::unordered_map<NumType, NumType> memory;
    while (std::getline(input, line)) {
      if (line.starts_with("mask")) masks = parseMaskForPart2(line);
      else {
        auto [address, value] = parseMem(line);
        for (const auto& mask : masks) memory[address | mask] = value;
      }
    }

    return std::accumulate(memory.begin(), memory.end(), (NumType)0,
                           [](const auto& total, const auto& pair) { return total + pair.second; });
  }

 private:
  std::vector<Bitmask> parseMaskForPart2(const std::string& line) {
    std::stringstream ss(line);
    std::string mask;
    ss >> mask >> mask >> mask;
    std::vector<std::string> masks;
    auto size = std::ranges::count(mask, 'X');
    masks.reserve(size);
    masks.push_back(mask);

    while (std::ranges::find(masks[0], 'X') != masks[0].end()) {
      auto changedBit = std::find(masks[0].begin(), masks[0].end(), 'X') - masks[0].begin();
      std::vector<std::string> certainMasks;
      certainMasks.reserve(1 << 10);

      for (auto& mask : masks) {
        mask[changedBit] = '0', certainMasks.push_back(mask);
        mask[changedBit] = '1', certainMasks.push_back(mask);
      }
      masks = certainMasks;
    }

    auto castToNumType = [](const auto& el) { return std::stoull(el); };
    std::vector<Bitmask> casted;
    std::ranges::transform(masks, std::back_inserter(casted), castToNumType);
    return casted;
  }
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
  std::cout << "Part 2: " << s.part2("test") << '\n';
  return 0;
}
