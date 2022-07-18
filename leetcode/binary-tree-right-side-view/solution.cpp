// link to the problem: https://leetcode.com/problems/binary-tree-right-side-view/
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

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Binary Tree Right Side View.
// Memory Usage: 12.9 MB, less than 6.99% of C++ online submissions for Binary Tree Right Side View.

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
  std::vector<int> rightSideView(TreeNode *root) {
    std::queue<TreeNode *> levelNodes;
    std::vector<int> result;

    if (root == nullptr) return result;
    levelNodes.push(root);
    while (!levelNodes.empty()) {
      std::queue<TreeNode *> nextLevelNodes;
      result.push_back(levelNodes.back()->val);
      while (!levelNodes.empty()) {
        TreeNode *node = levelNodes.front();
        levelNodes.pop();
        if (node->left != nullptr) nextLevelNodes.push(node->left);
        if (node->right != nullptr) nextLevelNodes.push(node->right);
      }
      levelNodes = nextLevelNodes;
    }
    return result;
  }
};

int main(int argc, char **argv) {}
