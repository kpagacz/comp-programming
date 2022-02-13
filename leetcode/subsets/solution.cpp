// link to the problem: https://leetcode.com/problems/subsets/
// What did I learn:
// * I had a bug where I was making the AND operation with "&&" instead of "&"...
// * I had an "off by one" bug where I was taking 2 to the power of nums.size() + 1 instead of nums.size()
#include <algorithm>
#include <array>
#include <cassert>
#include <cmath>
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
#include <vector>

class Solution {
 public:
  std::vector<std::vector<int>> subsets(std::vector<int>& nums) {
    int max_bits = std::pow(2, nums.size()) - 1;
    int it = 0;
    std::vector<int> current;
    std::vector<std::vector<int>> answer;
    while (it <= max_bits) {
      auto temp = it;
      int counter = 0;
      while(temp) {
        if (temp & 1) current.push_back(nums[counter]);
        temp = temp >> 1;
        counter++;
      }
      answer.push_back(current);
      current.clear();
      it++;
    }
    return answer;
  }
};

int main(int argc, char** argv) {}
