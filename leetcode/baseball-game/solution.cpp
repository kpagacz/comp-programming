// link to the problem: https://leetcode.com/problems/baseball-game/
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

class Solution {
 public:
  int calPoints(std::vector<std::string>& ops) {
    std::vector<int> scores;
    for (const auto& op : ops) {
      if (op == "C") {
        scores.pop_back();
      } else if (op == "D") {
        scores.push_back(scores.back() * 2);
      } else if (op == "+") {
        scores.push_back(scores[scores.size() - 1] + scores[scores.size() - 2]);
      } else {
        scores.push_back(std::stoi(op));
      }
    }
    return std::accumulate(scores.begin(), scores.end(), 0);
  }
};

int main(int argc, char** argv) {}
