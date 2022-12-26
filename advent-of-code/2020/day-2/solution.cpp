#include <cstdio>
#include <fstream>
#include <iostream>
#include <ranges>
#include <string>

class Solution {
 public:
  int part1(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string token;
    int answer = 0;
    while (input >> token) {
      int min, max;
      std::sscanf(token.c_str(), "%i-%i", &min, &max);
      input >> token;
      char pattern;
      std::sscanf(token.c_str(), "%c:", &pattern);
      input >> token;
      int occurrences = 0;
      for (const auto& c : token)
        if (c == pattern) occurrences++;
      if (occurrences >= min && occurrences <= max) answer++;
    }
    return answer;
  }

  int part2(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string token;
    int answer = 0;
    while (input >> token) {
      int min, max;
      std::sscanf(token.c_str(), "%i-%i", &min, &max);
      input >> token;
      char pattern;
      std::sscanf(token.c_str(), "%c:", &pattern);
      input >> token;
      bool inFirst = token[min - 1] == pattern, inSecond = token[max - 1] == pattern;
      if (inFirst ^ inSecond) answer++;
    }
    return answer;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
