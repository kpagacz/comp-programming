// link to the problem: https://leetcode.com/problems/k-diff-pairs-in-an-array/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

class Solution {
 public:
  int findPairs(std::vector<int>& nums, int k) {
    std::unordered_map<int, int> counting_sort;
    for (const auto& num : nums) {
      auto inserted = counting_sort.insert({num, 1});
      if (!inserted.second) inserted.first->second++;
    }

    int answer = 0;
    if (k == 0) {
      answer = std::count_if(counting_sort.begin(), counting_sort.end(), [](const auto& p) { return p.second >= 2; });
    } else {
      for (const auto& p : counting_sort) answer += counting_sort.count(p.first + k);
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
