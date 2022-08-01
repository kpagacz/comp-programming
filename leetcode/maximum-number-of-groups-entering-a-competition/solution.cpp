// link to the problem: https://leetcode.com/problems/maximum-number-of-groups-entering-a-competition/
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
  int maximumGroups(std::vector<int>& grades) {
    auto size = grades.size();
    int groups = 0;
    while (size >= groups + 1) {
      groups++;
      size -= groups;
    }
    return groups;
  }
};

int main(int argc, char** argv) {}
