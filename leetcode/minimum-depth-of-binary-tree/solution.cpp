// link to the problem: https://leetcode.com/problems/minimum-depth-of-binary-tree/
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

// Good enough for me:
// Runtime: 291 ms, faster than 88.80% of C++ online submissions for Minimum Depth of Binary Tree.
// Memory Usage: 144.7 MB, less than 43.56% of C++ online submissions for Minimum Depth of Binary Tree.

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
  int minDepth(TreeNode *root) {
    if (root == nullptr)
      return 0;
    else
      return minDepth(root, 1);
  }
  int minDepth(TreeNode *node, int depth) {
    if (node == nullptr) return INT_MAX;
    if (node->left == nullptr && node->right == nullptr) return depth;
    return std::min(minDepth(node->left, depth + 1), minDepth(node->right, depth + 1));
  }
};

int main(int argc, char **argv) {}
