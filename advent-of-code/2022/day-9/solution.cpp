#include <fstream>
#include <iostream>
#include <unordered_set>
#include <vector>

struct PairHash {
  template <class T1, class T2>
  std::size_t operator()(const std::pair<T1, T2>& pair) const {
    return (std::hash<T1>()(pair.first) << 16) ^ std::hash<T2>()(pair.second);
  }
};

class Solution {
 public:
  uint64_t part1(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string direction, stepsString;
    std::pair<int, int> tail{0, 0}, head{0, 0};
    std::unordered_set<std::pair<int, int>, PairHash> visitedPoints;
    while (input >> direction) {
      input >> stepsString;
      int steps = std::stoi(stepsString);

      std::pair<int, int> step;
      if (direction == "U") step = {-1, 0};
      else if (direction == "D") step = {1, 0};
      else if (direction == "L") step = {0, -1};
      else if (direction == "R") step = {0, 1};

      while (steps > 0) {
        head.first += step.first, head.second += step.second;
        moveTail(tail, head);
        visitedPoints.insert(tail);
        steps--;
      }
    }

    return visitedPoints.size();
  }
  uint64_t part2(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string direction, stepsString;
    std::vector<std::pair<int, int>> rope(10, {0, 0});
    std::unordered_set<std::pair<int, int>, PairHash> visitedPoints;
    while (input >> direction) {
      input >> stepsString;
      int steps = std::stoi(stepsString);

      std::pair<int, int> step;
      if (direction == "U") step = {-1, 0};
      else if (direction == "D") step = {1, 0};
      else if (direction == "L") step = {0, -1};
      else if (direction == "R") step = {0, 1};

      while (steps > 0) {
        rope[0].first += step.first, rope[0].second += step.second;
        for (int i = 1; i < rope.size(); i++) moveTail(rope[i], rope[i - 1]);
        visitedPoints.insert(rope.back());
        steps--;
      }
    }

    return visitedPoints.size();
  }

 private:
  void moveTail(std::pair<int, int>& tail, const std::pair<int, int> head) {
    int deltaX = std::abs(tail.first - head.first), deltaY = std::abs(tail.second - head.second);
    if (deltaX <= 1 && deltaY <= 1) return;
    if (deltaX == 2 && deltaY == 0) tail.first = (tail.first + head.first) / 2;
    if (deltaX == 0 && deltaY == 2) tail.second = (tail.second + head.second) / 2;
    if (deltaX == 2 && deltaY == 1) tail.first = (tail.first + head.first) / 2, tail.second = head.second;
    if (deltaX == 1 && deltaY == 2) tail.first = head.first, tail.second = (tail.second + head.second) / 2;
    if (deltaX == 2 && deltaY == 2)
      tail.first = (tail.first + head.first) / 2, tail.second = (tail.second + head.second) / 2;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
