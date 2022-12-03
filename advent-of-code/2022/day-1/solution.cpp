#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <sstream>
#include <vector>

class Solution {
 public:
  uint64_t mostCalories(std::string pathToInput) {
    std::fstream inputFile(pathToInput, std::ios_base::in);
    std::string line;
    std::vector<uint64_t> calories;
    while (std::getline(inputFile, line)) {
      if (line == "") continue;
      uint64_t elfCalories = 0;
      while (line != "") {
        elfCalories += std::stoull(line);
        std::getline(inputFile, line);
      }
      calories.push_back(elfCalories);
    }
    return *std::max_element(calories.begin(), calories.end());
  }

  uint64_t top3Calories(std::string pathToInput) {
    std::fstream inputFile(pathToInput, std::ios_base::in);
    std::string line;
    std::vector<uint64_t> calories;
    while (std::getline(inputFile, line)) {
      if (line == "") continue;
      uint64_t elfCalories = 0;
      while (line != "") {
        elfCalories += std::stoull(line);
        std::getline(inputFile, line);
      }
      calories.push_back(elfCalories);
    }

    std::sort(calories.begin(), calories.end(), std::greater<uint64_t>());
    return calories[0] + calories[1] + calories[2];
  }
};

int main() {
  Solution s;
  std::cout << "Part 1 - most calories: " << s.mostCalories("input") << '\n';
  std::cout << "Part 2 - sum of top 3: " << s.top3Calories("input") << '\n';
  return 0;
}
