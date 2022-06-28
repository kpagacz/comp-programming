// link to the problem:
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

// Good enough for me. A clever idea.
// Runtime: 12 ms, faster than 92.04% of C++ online submissions for Convert Sorted Array to Binary Search Tree.
// Memory Usage: 21.4 MB, less than 40.75% of C++ online submissions for Convert Sorted Array to Binary Search Tree.

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};
class Solution {
 public:
  TreeNode *sortedArrayToBST(std::vector<int> &nums) { return sortedArrayToBSTRecursive(nums, 0, nums.size()); }

  TreeNode *sortedArrayToBSTRecursive(const std::vector<int> &nums, const int begin, const int end) {
    if (begin == end) return nullptr;
    const int middle = (end + begin) / 2;
    TreeNode *newNode = new TreeNode(nums[middle]);
    newNode->left = sortedArrayToBSTRecursive(nums, begin, middle);
    newNode->right = sortedArrayToBSTRecursive(nums, middle + 1, end);
    return newNode;
  }
};

int main(int argc, char **argv) {}
