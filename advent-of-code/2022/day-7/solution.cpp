#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

// This was tricky because it turns out the directory names were not unique
// and my unordered_map could not deal with it...

class Solution {
 public:
  uint64_t part1(const std::string& pathToInput) {
    auto [sizes, _1, _2] = interpretInput(pathToInput);

    const uint64_t LIMIT = 100000;
    uint64_t answer = 0;
    for (const auto& size : sizes)
      if (size <= LIMIT) answer += size;
    return answer;
  }

  uint64_t part2(const std::string& pathToInput) {
    auto [sizes, _1, _2] = interpretInput(pathToInput);

    const uint64_t TOTAL_SIZE = 70000000;
    const uint64_t NEEDED_UNUSED = 30000000;

    uint64_t currentlyUnused = TOTAL_SIZE - sizes[0];
    if (currentlyUnused > NEEDED_UNUSED) return 0;
    uint64_t stillNeeded = NEEDED_UNUSED - currentlyUnused;

    std::sort(sizes.begin(), sizes.end());
    return *std::lower_bound(sizes.begin(), sizes.end(), stillNeeded);
  }

 private:
  std::tuple<std::vector<uint64_t>, std::vector<int>, std::unordered_map<std::string, int>> interpretInput(
      const std::string& pathToInput) {
    std::vector<uint64_t> sizes{0};
    std::vector<int> parents{-1};
    std::unordered_map<std::string, int> address{{"/", 0}};
    int currentDirectory = 0;
    std::fstream input(pathToInput, std::ios_base::in);
    std::vector<bool> alreadyLsed{false};

    std::string command;
    while (input >> command) {
      if (command == "$") continue;
      if (command == "cd") changeDirectory(currentDirectory, parents, input, address);
      if (command == "ls") readChildren(currentDirectory, parents, sizes, input, address, alreadyLsed);

    }

    return std::make_tuple(sizes, parents, address);
  }

  void changeDirectory(int& currentDirectory, const std::vector<int>& parents, std::fstream& input,
                       const std::unordered_map<std::string, int>& address) {
    std::string destination;
    input >> destination;
    if (destination == "/") {
      currentDirectory = 0;
      return;
    }
    if (destination == "..") {
      currentDirectory = parents[currentDirectory];
      return;
    }
    currentDirectory = address.at(destination + std::to_string(currentDirectory));
  }

  void readChildren(const int& currentDirectory, std::vector<int>& parents, std::vector<uint64_t>& sizes,
                    std::fstream& input, std::unordered_map<std::string, int>& address,
                    std::vector<bool>& alreadyLsed) {
    std::string firstToken, secondToken;
    while (input >> firstToken) {
      if (firstToken == "$") return;
      input >> secondToken;
      if (firstToken == "dir") {
        parents.push_back(currentDirectory);
        sizes.push_back(0);
        address[secondToken + std::to_string(currentDirectory)] = sizes.size() - 1;
        alreadyLsed.push_back(false);
      } else {
        if (alreadyLsed[currentDirectory]) continue;
        int directoryIt = currentDirectory;
        while (directoryIt != -1) {
          sizes[directoryIt] += std::stoull(firstToken);
          directoryIt = parents[directoryIt];
        }
      }
    }
    alreadyLsed[currentDirectory] = true;
  }
};

int main() {
  Solution s;
  // std::cout << "Test: " << s.part1("test") << '\n';
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
