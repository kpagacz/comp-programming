// link to the problem: https://leetcode.com/problems/find-players-with-zero-or-one-losses/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

// Runtime: 1591 ms, faster than 25.55% of C++ online submissions for Find Players With Zero or One Losses.
// Memory Usage: 193.2 MB, less than 23.38% of C++ online submissions for Find Players With Zero or One Losses.

class Solution {
 public:
  std::vector<std::vector<int>> findWinners(std::vector<std::vector<int>>& matches) {
    std::unordered_map<int, int> losses;
    std::set<int> players;

    for (const auto& match : matches) {
      losses[match[1]]++;
      players.insert(match[0]);
      players.insert(match[1]);
    }

    std::vector<std::vector<int>> answer(2, std::vector<int>());
    for (const auto& player : players) switch (losses[player]) {
        case 0:
          answer[0].push_back(player);
          break;
        case 1:
          answer[1].push_back(player);
        default:
          continue;
      }
    return answer;
  }
};

int main(int argc, char** argv) {}
