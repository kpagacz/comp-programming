// link to the problem: https://leetcode.com/problems/majority-element/
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
  int majorityElement(std::vector<int>& nums) {
    int majority_element = 0;
    int counter = 0;
    for (const auto& num : nums) {
      if (num == majority_element)
        counter++;
      else {
        counter--;
        if (counter <= 0) {
          majority_element = num;
          counter = 1;
        }
      }
    }
    return majority_element;
  }
};
int main(int argc, char** argv) {}
