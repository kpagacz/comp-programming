#include <algorithm>
#include <bitset>
#include <cassert>
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
      if (line.starts_with("mask")) std::tie(ones, zeros) = parseMask(line);
      else {
        auto [address, value] = parseMem(line);
        value |= ones;
        value &= zeros;
        memory[address] = value;
      }
    }

    auto values = memory | std::views::values | std::views::common;
    return std::accumulate(values.begin(), values.end(), (NumType)0);
  }
  NumType part2(const std::string& path) {
    std::unordered_map<NumType, NumType> memory;
    std::vector<std::size_t> floatingBitsPositions;
    std::vector<Bitmask> masks;

    std::fstream input(path, std::ios_base::in);
    std::string line;
    while (std::getline(input, line)) {
      if (line.starts_with("mask")) std::tie(floatingBitsPositions, masks) = parseMaskForPart2(line);
      else {
        const auto& [address, value] = parseMem(line);
        std::ranges::for_each(masks, [&](const auto& mask) {
          auto maskedAddress =
              std::accumulate(floatingBitsPositions.begin(), floatingBitsPositions.end(), address | mask,
                              [&](const auto& maskedAddress, const auto& pos) {
                                return ((mask & (1ull << pos))) ? maskedAddress : maskedAddress & (~(1ull << pos));
                              });
          memory[maskedAddress] = value;
        });
      }
    }

    auto values = memory | std::views::values | std::views::common;
    return std::accumulate(values.begin(), values.end(), (NumType)0);
  }

 private:
  std::pair<std::vector<std::size_t>, std::vector<Bitmask>> parseMaskForPart2(const std::string& line) {
    std::stringstream ss(line);
    std::string mask;
    ss >> mask >> mask >> mask;

    std::vector<std::size_t> floating;
    for (auto idx : std::views::iota(0u, mask.size()))
      if (mask[idx] == 'X') floating.push_back(mask.size() - 1 - idx);

    // Masks
    std::vector<std::string> masks = {mask};
    while (std::ranges::find(masks[0], 'X') != masks[0].end()) {
      std::vector<std::string> newMasks;
      newMasks.reserve(masks.size() * 2);
      auto foundX = masks[0].find('X');
      for (auto& mask : masks) {
        mask[foundX] = '0', newMasks.push_back(mask);
        mask[foundX] = '1', newMasks.push_back(mask);
      }
      masks = newMasks;
    }

    assert(masks.size() == (1ull << std::ranges::count(mask, 'X')));

    std::vector<Bitmask> cast;
    std::ranges::transform(masks, std::back_inserter(cast),
                           [&](const auto& mask) { return std::stoull(mask, nullptr, 2); });
    return {floating, cast};
  }
  std::pair<Bitmask, Bitmask> parseMask(const std::string& line) {
    std::stringstream ss(line);
    std::string mask;
    ss >> mask >> mask >> mask;
    return convertMaskToPair(mask);
  }
  std::pair<Bitmask, Bitmask> convertMaskToPair(const std::string& mask) {
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
  std::cout << "Part 2: " << s.part2("input") << '\n';

  return 0;
}
