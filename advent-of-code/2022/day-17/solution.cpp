#include <cassert>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

template <class T>
struct PairHash {
  std::size_t operator()(const std::pair<T, T>& key) const {
    std::size_t hash = 0;
    hash = std::hash<int>()(key.first) << 16;
    hash ^= std::hash<int>()(key.second);
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
      // std::cout << "Rock: " << rocks << '\n';
      auto didShift = true;
      if (rocks % shapes.size() == 0) std::cout << "Rock: " << rocks << " jets: " << jet << '\n';
      while (didShift) {
        // std::cout << "Start: " << start.first << " " << start.second << '\n';
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
        if (jet % 40 == 0) std::cout << "Height: " << height(heights) << '\n';
        jet++;
        didShift = shiftShape(start, shapes[rocks % shapes.size()], DOWN, allPoints);
      }
      // std::cout << "Landed at: " << start.first << " " << start.second << '\n';
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
    auto jets = readInput(path);
    auto [heights, allPoints] = prepareStructures<int64_t>();
    int64_t rocks = 0;
    int64_t jet = 0;

    std::cout << shapes.size() << " " << jets.size() << " ";
    std::cout << (int)std::lcm(shapes.size(), jets.size());

    std::cout << '\n';
    return 0;
  }

 private:
  template <class T>
  std::tuple<std::vector<T>, std::unordered_set<std::pair<T, T>, PairHash<T>>> prepareStructures() {
    auto heights = std::vector<T>(7, -1);
    auto allPoints = std::unordered_set<std::pair<T, T>, PairHash<T>>();
    return {heights, allPoints};
  }

  bool shiftShape(std::pair<int64_t, int64_t>& root, const std::vector<std::pair<int64_t, int64_t>>& shape,
                  const std::pair<int64_t, int64_t> shift,
                  const std::unordered_set<std::pair<int64_t, int64_t>, PairHash<int64_t>>& allPoints) {
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
  std::cout << "Part 2: " << s.part2("test") << '\n';
  return 0;
}
