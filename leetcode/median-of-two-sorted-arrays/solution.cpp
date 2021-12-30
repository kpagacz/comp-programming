// link to the problem
// https://leetcode.com/problems/median-of-two-sorted-arrays/
#include <algorithm>
#include <iostream>
#include <vector>

class Solution {
 public:
  double findMedianSortedArrays(std::vector<int> nums1,
                                std::vector<int> nums2) {
    if (nums1.size() > nums2.size()) return findMedianSortedArrays(nums2, nums1);

    if (nums1.size() == 0) {
      return nums2.size() % 2
                 ? nums2[nums2.size() / 2]
                 : 1.0 *
                       (nums2[nums2.size() / 2] + nums2[nums2.size() / 2 - 1]) /
                       2;
    }

    // nums1 is the shorter array and has some elements in it
    // invariant in every loop - the number of elements smaller than right1 and
    // right2 is the same - half of the number of all the elements
    int left1 = (nums1.size() + 1) / 2 - 1;
    int right1 = left1 + 1;
    int min_bound = 0, max_bound = nums1.size() - 1;
    const int elements_below_median = (nums1.size() + nums2.size()) / 2;
    const int total_size = nums1.size() + nums2.size();
    int elements_left = elements_below_median - left1 - 1;
    int left2 = elements_left - 1, right2 = elements_left;

    while (true) {
      if (left1 == -1 && nums2[left2] <= nums1[right1]) break;
      if (right1 == nums1.size() && nums1[left1] <= nums2[right2]) break;
      if (left1 != -1 && right1 != nums1.size() &&
          nums1[left1] <= nums2[right2] && nums2[left2] <= nums1[right1])
        break;
      if (left1 != -1 && nums1[left1] > nums2[right2]) {
        max_bound = right1;
        if (left1 == 0) left1 = -1; else left1 = (left1 + min_bound) / 2;
        right1 = left1 + 1;
      } else {
        min_bound = left1;
        if (right1 == nums1.size() - 1) right1 = nums1.size(); else right1 = (right1 + max_bound + 1) / 2;
        left1 = right1 - 1;
      }

      elements_left = elements_below_median - right1;
      left2 = elements_left - 1;
      right2 = elements_left;
    }

    int left, right;
    if (left1 != -1 && left2 != -1) {
      left = std::max(nums1[left1], nums2[left2]);
    } else {
      if (left1 == -1) left = nums2[left2];
      if (left2 == -1) left = nums1[left1];
    }

    if (right1 != nums1.size() && right2 != nums2.size()) {
      right = std::min(nums1[right1], nums2[right2]);
    } else {
      if (right1 == nums1.size()) right = nums2[right2];
      if (right2 == nums2.size()) right = nums1[right1];
    }
    if (total_size % 2) {
      return right;
    } else {
      return 1.0 * (left + right) / 2;
    }
  }
};

int main() {
  Solution s;
  std::cout << s.findMedianSortedArrays({1,2,3,4}, {5,6,7,8});
}
