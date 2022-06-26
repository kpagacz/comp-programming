// link to the problem: https://leetcode.com/problems/merge-sorted-array/
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

// I hate reimplementing functions from the standard library.
// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Merge Sorted Array.
// Memory Usage: 9.2 MB, less than 29.05% of C++ online submissions for Merge Sorted Array.

class Solution {
 public:
  void merge(std::vector<int>& nums1, int m, std::vector<int>& nums2, int n) {
    std::vector<int> sorted(m + n);
    std::merge(nums1.begin(), nums1.begin() + m, nums2.begin(), nums2.begin() + n, sorted.begin());
    nums1 = std::move(sorted);
  }

  void mergeReimplemented(std::vector<int>& nums1, int m, std::vector<int>& nums2, int n) {
    std::vector<int> sorted(m + n);
    auto nums1It = nums1.begin(), nums1End = nums1.begin() + m, nums2It = nums2.begin(), nums2End = nums2.begin() + n;
    auto sortedIt = sorted.begin();
    for (; nums1It != nums1End; ++sortedIt) {
      if (nums2It == nums2End) {
        std::copy(nums1It, nums1End, sortedIt);
        break;
      }
      if (*nums1It < *nums2It) {
        *sortedIt = *nums1It;
        ++nums1It;
      } else {
        *sortedIt = *nums2It;
        ++nums2It;
      }
    }
    std::copy(nums2It, nums2End, sortedIt);
    nums1 = std::move(sorted);
  }
};

int main(int argc, char** argv) {}
