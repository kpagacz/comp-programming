#include <cassert>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "utils.cc"

template <class Ch, class Tr, class... Args>
auto& operator<<(std::basic_ostream<Ch, Tr>& os, std::tuple<Args...> const& t) {
  std::apply([&os](auto&&... args) { ((os << args << " "), ...); }, t);
  return os;
}

template <class T>
struct PairHash {
  std::size_t operator()(const std::pair<T, T>& key) const {
    std::size_t hash = 0;
    hash = std::hash<T>()(key.first) << 16;
    hash ^= std::hash<T>()(key.second);
    return hash;
  }
};

class Solution {
 public:
  int part1(const std::string& path) {
    auto shifts = readInput(path);
    auto [heights, allPoints] = prepareStructures<int64_t>();
    int rocks = 0;
    int jet = 0;

    while (rocks < 2022) {
      auto start = getStartingPoint<int64_t>(heights);
      auto didShift = true;
      while (didShift) {
        switch (shifts[jet % shifts.size()]) {
          case '>':
            shiftShape(start, shapes[rocks % shapes.size()], RIGHT, allPoints);
            break;
          case '<':
            shiftShape(start, shapes[rocks % shapes.size()], LEFT, allPoints);
            break;
          default:
            assert(false);
        }
        jet++;
        didShift = shiftShape(start, shapes[rocks % shapes.size()], DOWN, allPoints);
      }
      for (const auto& point : shapes[rocks % shapes.size()]) {
        auto added = add(start, point);
        heights[added.second] = std::max(heights[added.second], added.first);
        allPoints.insert(added);
      }
      rocks++;
    }

    return *std::max_element(heights.begin(), heights.end()) + 1;
  }

  int64_t part2(const std::string& path) {
    auto shifts = readInput(path);
    auto [heights, allPoints] = prepareStructures<int64_t>();
    int rocks = 0;
    int jet = 0;

    using Key = std::tuple<int, int, int, int, int, int, int, int>;
    using Value = std::vector<std::tuple<int, int>>;  // rocks fallen, height

    std::unordered_map<Key, Value, utils::TupleHash<int, int, int, int, int, int, int, int>> cache;
    auto constructKey = [&](const int& jet, const std::vector<int64_t>& heights, const auto& allPoints) {
      std::vector<int> masks;
      auto maxHeight = height(heights);
      for (int i = 0; i < heights.size(); i++) {
        int mask = 0;
        for (int j = 0; j < 30; j++)
          if (allPoints.contains({maxHeight - j, i})) mask |= 1 << j;
        masks.push_back(mask);
      }
      return std::make_tuple(jet, masks[0], masks[1], masks[2], masks[3], masks[4], masks[5], masks[6]);
    };

    while (rocks < 4000) {
      auto start = getStartingPoint<int64_t>(heights);
      auto didShift = true;

      // cache
      auto key = constructKey(jet, heights, allPoints);
      cache[key].push_back({rocks, height(heights)});

      while (didShift) {
        switch (shifts[jet % shifts.size()]) {
          case '>':
            shiftShape(start, shapes[rocks % shapes.size()], RIGHT, allPoints);
            break;
          case '<':
            shiftShape(start, shapes[rocks % shapes.size()], LEFT, allPoints);
            break;
          default:
            assert(false);
        }
        jet = (jet + 1) % shifts.size();
        didShift = shiftShape(start, shapes[rocks % shapes.size()], DOWN, allPoints);
      }
      for (const auto& point : shapes[rocks % shapes.size()]) {
        auto added = add(start, point);
        heights[added.second] = std::max(heights[added.second], added.first);
        allPoints.insert(added);
      }
      rocks++;
    }

    // Cycle detection
    int cycleBaseRocks, cycleBaseHeight, cycleHeightDelta, cycleRockDelta;
    for (const auto& [key, series] : cache) {
      auto s = series.size();
      bool found = false;
      for (int i = 0; i < s && !found; i++)
        for (int j = i + 1; j < s && !found; j++)
          for (int k = j + 1; k < s && !found; k++) {
            const auto [firstRocks, firstHeight] = series[i];
            const auto [secondRocks, secondHeight] = series[j];
            const auto [thirdRocks, thirdHeight] = series[k];
            if (secondRocks - firstRocks == thirdRocks - secondRocks &&
                secondHeight - firstHeight == thirdHeight - secondHeight) {
              std::cout << "\nCYCLE DETECTED:\n";
              std::cout << "Cycle begins with rock:" << firstRocks << " at height:" << firstHeight << '\n';
              std::cout << "Repeats at rock:" << secondRocks << " at height:" << secondHeight << '\n';
              std::cout << "Shows again at rock:" << thirdRocks << " at height:" << thirdHeight << '\n';
              found = true;
              cycleBaseRocks = firstRocks, cycleBaseHeight = firstHeight, cycleRockDelta = secondRocks - firstRocks,
              cycleHeightDelta = secondHeight - firstHeight;
            }
          }
      if (found) break;
    }

    // run again
    std::tie(heights, allPoints) = prepareStructures<int64_t>();
    rocks = 0;
    jet = 0;
    const int64_t TO_THROW = 1e12;
    int toThrowAdditional = (TO_THROW - cycleBaseRocks) % cycleRockDelta;
    while (rocks < cycleBaseRocks + toThrowAdditional) {
      auto start = getStartingPoint<int64_t>(heights);
      auto didShift = true;

      while (didShift) {
        switch (shifts[jet % shifts.size()]) {
          case '>':
            shiftShape(start, shapes[rocks % shapes.size()], RIGHT, allPoints);
            break;
          case '<':
            shiftShape(start, shapes[rocks % shapes.size()], LEFT, allPoints);
            break;
          default:
            assert(false);
        }
        jet = (jet + 1) % shifts.size();
        didShift = shiftShape(start, shapes[rocks % shapes.size()], DOWN, allPoints);
      }
      for (const auto& point : shapes[rocks % shapes.size()]) {
        auto added = add(start, point);
        heights[added.second] = std::max(heights[added.second], added.first);
        allPoints.insert(added);
      }
      rocks++;
    }
    auto additionalHeight = height(heights) - cycleBaseHeight;
    auto finalHeight =
        cycleBaseHeight + cycleHeightDelta * ((TO_THROW - cycleBaseRocks) / cycleRockDelta) + additionalHeight + 1;

    return finalHeight;
  }

 private:
  template <class T>
  std::tuple<std::vector<T>, std::unordered_set<std::pair<T, T>, utils::TupleHash<int64_t, int64_t>>>
  prepareStructures() {
    auto heights = std::vector<T>(7, -1);
    auto allPoints = std::unordered_set<std::pair<T, T>, utils::TupleHash<int64_t, int64_t>>();
    return {heights, allPoints};
  }

  bool shiftShape(
      std::pair<int64_t, int64_t>& root, const std::vector<std::pair<int64_t, int64_t>>& shape,
      const std::pair<int64_t, int64_t> shift,
      const std::unordered_set<std::pair<int64_t, int64_t>, utils::TupleHash<int64_t, int64_t>>& allPoints) {
    auto shiftedRoot = add(root, shift);
    for (const auto& point : shape) {
      auto shifted = add(shiftedRoot, point);
      if (shifted.second == LEFT_BORDER || shifted.second == RIGHT_BORDER) return false;
      if (shifted.first == -1) return false;
      if (allPoints.contains(shifted)) return false;
    }
    root = shiftedRoot;
    return true;
  }

  template <class T>
  std::pair<T, T> add(const std::pair<T, T>& first, const std::pair<T, T>& second) {
    return {first.first + second.first, first.second + second.second};
  }

  template <class T>
  std::pair<T, T> getStartingPoint(const std::vector<T>& heights) {
    return {*std::max_element(heights.begin(), heights.end()) + 4, START_WIDTH};
  }

  int64_t height(const std::vector<int64_t>& heights) {
    return *std::max_element(heights.begin(), heights.end());
  }

  const std::vector<std::vector<std::pair<int64_t, int64_t>>> shapes{{{0, 0}, {0, 1}, {0, 2}, {0, 3}},
                                                                     {{0, 1}, {1, 0}, {1, 1}, {1, 2}, {2, 1}},
                                                                     {{0, 0}, {0, 1}, {0, 2}, {1, 2}, {2, 2}},
                                                                     {{0, 0}, {1, 0}, {2, 0}, {3, 0}},
                                                                     {{0, 0}, {0, 1}, {1, 0}, {1, 1}}};
  const int START_WIDTH = 2;
  const int LEFT_BORDER = -1;
  const int RIGHT_BORDER = 7;
  const std::pair<int64_t, int64_t> LEFT{0, -1}, RIGHT{0, 1}, DOWN{-1, 0};

  std::string readInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    input >> line;
    return line;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
