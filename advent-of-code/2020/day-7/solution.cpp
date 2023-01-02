#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using NumType = int;
class Solution {
 public:
  NumType part1(const std::string& path) {
    auto bags = parseInput(path);

    const auto getContainingBags = [&bags = bags](const auto& bag) {
      auto recursion = [&](const auto& bag, const auto& self) {
        if (!bags.contains(bag)) return std::unordered_set<std::string>{bag};
        const auto& containingBags = bags.at(bag);
        auto ancestorBags = std::transform_reduce(
            containingBags.begin(), containingBags.end(), std::unordered_set<std::string>(),
            [&](const auto& first, const auto& second) {
              std::unordered_set<std::string> res;
              res.insert(first.begin(), first.end());
              res.insert(second.begin(), second.end());
              return res;
            },
            [&](const auto& containingBag) { return self(containingBag.first, self); });
        ancestorBags.insert(bag);
        return ancestorBags;
      };
      return recursion(bag, recursion);
    };

    auto answer = getContainingBags("shinygold");
    return answer.size() - 1;
  }

  NumType part2(const std::string& path) {
    auto bags = parseInputPart2(path);

    const auto countBagWeight = [&bags = bags](const auto& bagName) {
      auto countBugsRec = [&](const auto& bagName, const auto& self) {
        const auto& childBags = bags[bagName];
        if (childBags.empty()) return NumType(1);
        else
          return std::transform_reduce(
              childBags.begin(), childBags.end(), (NumType)1, std::plus<NumType>(),
              [&](const auto& childBag) { return childBag.second * self(childBag.first, self); });
      };

      return countBugsRec(bagName, countBugsRec);
    };

    return countBagWeight("shinygold") - 1;
  }

 private:
  using Bags = std::unordered_map<std::string, std::vector<std::pair<std::string, NumType>>>;
  Bags parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    Bags output;
    while (std::getline(input, line)) {
      std::stringstream ss(line);
      std::string token;
      std::string containingBag;

      ss >> containingBag >> token;
      containingBag += token;

      for (int i = 0; i < 2; i++) ss >> token;

      while (ss >> token) {
        if (token == "no") break;
        NumType containingNum = std::stoi(token);
        std::string bag;
        ss >> bag >> token;
        bag += token;
        output[bag].push_back({containingBag, containingNum});
        ss >> token;
      }
    }

    return output;
  }

  Bags parseInputPart2(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;

    Bags bags;
    while (std::getline(input, line)) {
      std::stringstream ss(line);
      std::string token;
      std::string containingBag;

      ss >> containingBag >> token;
      containingBag += token;

      for (int i = 0; i < 2; i++) ss >> token;

      while (ss >> token) {
        if (token == "no") break;
        NumType containingNum = std::stoi(token);
        std::string bag;
        ss >> bag >> token;
        bag += token;
        bags[containingBag].push_back({bag, containingNum});
        ss >> token;
      }
    }

    return bags;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
