#include <fstream>
#include <iostream>
#include <vector>

class Solution {
 public:
  std::string part1(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    auto crates = parseInitialPosition(input);

    for (std::string line; std::getline(input, line);) {
      auto [howMany, from, to] = parseAction(line);
      for (int i = 0; i < howMany; i++) crates[to].push_back(crates[from].back()), crates[from].pop_back();
    }

    std::string answer;
    for (const auto& stack : crates) answer += stack.back();

    replaceAll(answer, "[", "");
    replaceAll(answer, "]", "");
    return answer;
  }

  std::string part2(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    auto crates = parseInitialPosition(input);

    for (std::string line; std::getline(input, line);) {
      auto [howMany, from, to] = parseAction(line);
      crates[to].insert(crates[to].end(), std::make_move_iterator(crates[from].end() - howMany),
                        std::make_move_iterator(crates[from].end()));
      crates[from].erase(crates[from].end() - howMany, crates[from].end());
    }

    std::string answer;
    for (const auto& stack : crates) answer += stack.back();
    replaceAll(answer, "[", "");
    replaceAll(answer, "]", "");
    return answer;
  }

 private:
  std::vector<std::vector<std::string>> parseInitialPosition(std::fstream& input) {
    std::vector<std::vector<std::string>> crates(9, std::vector<std::string>());
    std::string line;
    while (std::getline(input, line)) {
      if (std::isdigit(line[1])) {
        std::getline(input, line);
        break;
      }

      for (int i = 0; i < line.size(); i += 4) {
        std::string crate = line.substr(i, 3);
        if (crate != "   ") crates[i / 4].push_back(crate);
      }
    }

    for (auto& crate : crates) std::reverse(crate.begin(), crate.end());
    return crates;
  }

  std::tuple<int, int, int> parseAction(const std::string& action) {
    int howMany, from, to;
    std::sscanf(action.c_str(), "move %d from %d to %d", &howMany, &from, &to);
    from--;
    to--;
    return {howMany, from, to};
  }

  void replaceAll(std::string& str, const std::string& what, const std::string& with) {
    if (what.empty()) return;
    std::size_t pos;
    while ((pos = str.find(what)) != std::string::npos) str.replace(pos, what.size(), with);
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
