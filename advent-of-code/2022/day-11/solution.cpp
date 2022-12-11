#include <iostream>
#include <numeric>
#include <queue>
#include <string>
#include <vector>

class Solution {
 public:
  uint64_t part1() {
    std::vector<std::vector<uint64_t>> monkeyItems = items;
    std::vector<uint64_t> inspectedItemsCount(monkeyItems.size(), 0);

    const int ROUNDS = 20;
    for (auto i{0}; i < ROUNDS; i++)
      for (auto monkey{0}; monkey < monkeyItems.size(); monkey++)
        while (!monkeyItems[monkey].empty()) {
          inspectedItemsCount[monkey]++;
          auto item = monkeyItems[monkey].back();
          monkeyItems[monkey].pop_back();
          item = monkeyInspectOperation(monkey, item);
          item /= 3;
          monkeyItems[receivingMonkey(monkey, item)].push_back(item);
        }

    std::sort(inspectedItemsCount.begin(), inspectedItemsCount.end());
    return inspectedItemsCount[inspectedItemsCount.size() - 2] * inspectedItemsCount.back();
  }

  uint64_t part2() {
    std::vector<std::vector<uint64_t>> monkeyItems = items;
    std::vector<uint64_t> inspectedItemsCount(monkeyItems.size(), 0);

    const int ROUNDS = 10000;
    auto MODULO = std::accumulate(monkeyTestDivisors.begin(), monkeyTestDivisors.end(), 1ull, std::multiplies<uint64_t>());
    for (auto i{0}; i < ROUNDS; i++)
      for (auto monkey{0}; monkey < monkeyItems.size(); monkey++)
        while (!monkeyItems[monkey].empty()) {
          inspectedItemsCount[monkey]++;
          auto item = monkeyItems[monkey].back();
          monkeyItems[monkey].pop_back();
          item = monkeyInspectOperation(monkey, item);
          item %= MODULO;
          monkeyItems[receivingMonkey(monkey, item)].push_back(item);
        }

    std::sort(inspectedItemsCount.begin(), inspectedItemsCount.end());
    return inspectedItemsCount[inspectedItemsCount.size() - 2] * inspectedItemsCount.back();
  }

 private:
  const std::vector<uint64_t> monkeyTestDivisors{11, 5, 7, 2, 17, 13, 3, 19};
  const std::vector<int> receivingMonkeyTestTrue{2, 7, 1, 0, 3, 3, 1, 4};
  const std::vector<int> receivingMonkeyTestFalse{6, 4, 7, 6, 5, 0, 2, 5};

  uint64_t monkeyInspectOperation(const int& monkey, const uint64_t& worry) const {
    switch (monkey) {
      case 0:
        return worry * 7;
      case 1:
        return worry * 13;
      case 2:
        return worry + 1;
      case 3:
        return worry * worry;
      case 4:
        return worry + 7;
      case 5:
        return worry + 6;
      case 6:
        return worry + 4;
      case 7:
        return worry + 8;
      default:
        return -1;
    }
  }

  int receivingMonkey(const int& from, const uint64_t& worry) const {
    if (worry % monkeyTestDivisors[from] == 0) return receivingMonkeyTestTrue[from];
    else return receivingMonkeyTestFalse[from];
  }

  const std::vector<std::vector<uint64_t>> items{
      {54, 82, 90, 88, 86, 54},         {91, 65},         {62, 54, 57, 92, 83, 63, 63}, {67, 72, 68},
      {68, 89, 90, 86, 84, 57, 72, 84}, {79, 83, 64, 58}, {96, 72, 89, 70, 88},         {79}};
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1() << '\n';
  std::cout << "Part 2: " << s.part2() << '\n';
  return 0;
}
