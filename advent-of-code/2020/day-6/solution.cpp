#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

using NumType = int;
class Solution {
 public:
  NumType part1(const std::string& path) {
    auto groups = parseInput(path);

    auto countYesAnswers = [&](const auto& group) {
      std::vector<bool> yesAnswers(26, false);
      for (const auto& answers : group)
        for (const auto& question : answers) yesAnswers[question - 'a'] = true;
      return std::count(yesAnswers.begin(), yesAnswers.end(), true);
    };

    return std::transform_reduce(groups.begin(), groups.end(), (NumType)0, std::plus<NumType>(), countYesAnswers);
  }

  NumType part2(const std::string& path) {
    auto groups = parseInput(path);

    auto countYesAnswers = [&](const auto& group) {
      std::vector<bool> yesAnswers(26, true);
      for (const auto& answers : group) {
        std::vector<bool> personsYesAnswers(26, false);
        for (const auto& question : answers) personsYesAnswers[question - 'a'] = true;
        std::transform(yesAnswers.begin(), yesAnswers.end(), personsYesAnswers.begin(), yesAnswers.begin(),
                       [&](const auto& first, const auto& second) { return first && second; });
      }
      return std::count(yesAnswers.begin(), yesAnswers.end(), true);
    };

    return std::transform_reduce(groups.begin(), groups.end(), (NumType)0, std::plus<NumType>(), countYesAnswers);
  }

 private:
  std::vector<std::vector<std::string>> parseInput(const std::string_view& path) {
    std::vector<std::vector<std::string>> groups;
    std::fstream input(path, std::ios_base::in);
    std::string line;
    std::vector<std::string> group;
    while (std::getline(input, line))
      if (line == "") groups.push_back(group), group.clear();
      else group.push_back(line);
    groups.push_back(group);

    return groups;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
