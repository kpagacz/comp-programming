// link to the problem: https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/
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
  bool validPartition(std::vector<int>& nums) {
    std::vector<bool> isValidPartition(nums.size(), false);
    isValidPartition[1] = nums[0] == nums[1];
    if (nums.size() == 2) return isValidPartition[nums.size() - 1];
    isValidPartition[2] = (nums[0] == nums[1] && nums[1] == nums[2]) || (nums[0] + 1 == nums[1] && nums[0] + 2 == nums[2]);

    for (int i = 3; i < nums.size(); ++i) {
      isValidPartition[i] = ((nums[i] == nums[i - 1]) && isValidPartition[i - 2]);
      isValidPartition[i] = isValidPartition[i] || (nums[i] == nums[i - 1] && nums[i - 1] == nums[i - 2] && isValidPartition[i - 3]);
      isValidPartition[i] =
          isValidPartition[i] || (nums[i] == nums[i - 1] + 1 && nums[i] == nums[i - 2] + 2 && isValidPartition[i - 3]);
    }

    return isValidPartition[nums.size() - 1];
  }
};

int main(int argc, char** argv) {
  std::vector<std::vector<int>> cases{{1, 1, 1, 1, 2, 3}, {803201, 803201, 803201, 803201, 803202, 803203}};
  Solution s;
  for (auto& test : cases) {
    std::cout << "CASE: ";
    std::copy(test.begin(), test.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
    std::cout << std::boolalpha << s.validPartition(test) << '\n';
  }
}
