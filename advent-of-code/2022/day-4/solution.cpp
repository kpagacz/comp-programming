#include <cstdio>
#include <fstream>
#include <iostream>

class Solution {
 public:
  uint64_t part1(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string line;
    uint64_t answer = 0;
    while (input >> line) {
      uint64_t firstSectionBegin, firstSectionEnd, secondSectionBegin, secondSectionEnd;
      std::sscanf(line.c_str(), "%llu-%llu,%llu-%llu", &firstSectionBegin, &firstSectionEnd, &secondSectionBegin,
                  &secondSectionEnd);
      if ((firstSectionBegin <= secondSectionBegin && firstSectionEnd >= secondSectionEnd) ||
          (firstSectionBegin >= secondSectionBegin && firstSectionEnd <= secondSectionEnd))
        answer++;
    }

    return answer;
  }

  uint64_t part2(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string line;
    uint64_t overlapping = 0;

    while (input >> line) {
      uint64_t firstSectionBegin, firstSectionEnd, secondSectionBegin, secondSectionEnd;
      std::sscanf(line.c_str(), "%llu-%llu,%llu-%llu", &firstSectionBegin, &firstSectionEnd, &secondSectionBegin,
                  &secondSectionEnd);
      if ((firstSectionBegin >= secondSectionBegin && firstSectionBegin <= secondSectionEnd) ||
          (firstSectionEnd >= secondSectionBegin && firstSectionEnd <= secondSectionEnd) ||
          (firstSectionBegin <= secondSectionBegin && firstSectionEnd >= secondSectionEnd))
        overlapping++;
    }

    return overlapping;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
