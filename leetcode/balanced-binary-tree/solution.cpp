// link to the problem: https://leetcode.com/problems/balanced-binary-tree/
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

// Good enough
// Runtime: 7 ms, faster than 98.75% of C++ online submissions for Balanced Binary Tree.
// Memory Usage: 20.8 MB, less than 95.21% of C++ online submissions for Balanced Binary Tree.

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
  bool isBalanced(TreeNode *root) {
    if (root == nullptr) return true;
    return std::abs(height(root->left, 0) - height(root->right, 0)) <= 1 && isBalanced(root->left) &&
           isBalanced(root->right);
  }

 private:
  int height(TreeNode *root, int currentHeight) {
    if (root == nullptr) return currentHeight;
    return std::max(height(root->left, currentHeight + 1), height(root->right, currentHeight + 1));
  }
};

int main(int argc, char **argv) {}
