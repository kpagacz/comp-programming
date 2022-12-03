#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

class Solution {
 public:
  uint64_t part1(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string rucksack;
    uint64_t sumOfPriorities = 0;
    while (input >> rucksack) {
      std::string firstCompartment = rucksack.substr(0, rucksack.size() / 2);
      std::string secondCompartment = rucksack.substr(rucksack.size() / 2);
      std::sort(firstCompartment.begin(), firstCompartment.end());
      std::sort(secondCompartment.begin(), secondCompartment.end());
      std::vector<char> commonItems;
      std::set_intersection(firstCompartment.begin(), firstCompartment.end(), secondCompartment.begin(),
                            secondCompartment.end(), std::back_inserter(commonItems));
      const char& commonItem = commonItems[0];
      if (std::isupper(commonItem)) sumOfPriorities += commonItem - 'A' + 27;
      else sumOfPriorities += commonItem - 'a' + 1;
    }
    return sumOfPriorities;
  }

  uint64_t part2(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string rucksack;
    uint64_t sumOfPriorities = 0;
    while (input >> rucksack) {
      std::string secondRucksack, thirdRucksack;
      input >> secondRucksack >> thirdRucksack;
      for (auto sack : {&rucksack, &secondRucksack, &thirdRucksack}) std::sort(sack->begin(), sack->end());
      std::vector<char> commonItems;
      std::set_intersection(rucksack.begin(), rucksack.end(), secondRucksack.begin(), secondRucksack.end(),
                            std::back_inserter(commonItems));
      std::sort(commonItems.begin(), commonItems.end());
      std::set_intersection(commonItems.begin(), commonItems.end(), thirdRucksack.begin(), thirdRucksack.end(),
                            std::back_inserter(commonItems));
      const char& commonItem = commonItems.back();
      if (std::isupper(commonItem)) sumOfPriorities += commonItem - 'A' + 27;
      else sumOfPriorities += commonItem - 'a' + 1;
    }
    return sumOfPriorities;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
}
