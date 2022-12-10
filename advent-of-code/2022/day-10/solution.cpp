#include <fstream>
#include <iostream>
#include <string>
#include <unordered_set>

class Solution {
 public:
  int64_t part1(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    int64_t x = 1;
    int64_t sum = 0;
    uint64_t cycle = 0;
    std::string command;
    while (input >> command && cycle <= 220) {
      cycle++;
      addToSum(sum, x, cycle);
      if (command == "addx") {
        std::string value;
        input >> value;
        int64_t toAdd = std::stoll(value);
        cycle++;
        addToSum(sum, x, cycle);
        x += toAdd;
      }
    }
    return sum;
  }

  void part2(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    int64_t x = 1;
    int cycle = -1;
    std::string command;
    while (input >> command && cycle <= 240) {
      cycle++;
      if (cycle % 40 == 0) std::cout << '\n';
      draw(cycle, x);
      if (command == "addx") {
        std::string value;
        input >> value;
        int64_t toAdd = std::stoll(value);
        cycle++;
        if (cycle % 40 == 0) std::cout << '\n';
        draw(cycle, x);
        x += toAdd;
      }
    }
    std::cout << '\n';
  }

 private:
  void addToSum(int64_t& sum, const int64_t& registerValue, const uint64_t& cycle) {
    if (importantCycle.contains(cycle)) sum += registerValue * cycle;
  }
  void draw(uint64_t cycle, int64_t spritePosition) {
    int crtPosition = cycle % 40;
    if ((crtPosition - spritePosition) * (crtPosition - spritePosition) <= 1) std::cout << "#";
    else std::cout << ".";
  }
  const std::unordered_set<uint64_t> importantCycle{20, 60, 100, 140, 180, 220};
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  s.part2("input");
  return 0;
}
