// link to the problem: https://leetcode.com/problems/node-with-highest-edge-score/
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

// Runtime: 233 ms, faster than 82.67% of C++ online submissions for Node With Highest Edge Score.
// Memory Usage: 109.8 MB, less than 94.33% of C++ online submissions for Node With Highest Edge Score.

class Solution {
 public:
  int edgeScore(std::vector<int>& edges) {
    std::vector<int64_t> scores(edges.size(), 0);
    for (int i = 0; i < edges.size(); ++i) scores[edges[i]] += i;
    return std::max_element(scores.begin(), scores.end()) - scores.begin();
  }
};

int main(int argc, char** argv) {}
