// link to the problem: https://leetcode.com/problems/binary-tree-level-order-traversal/
#include <algorithm>
#include <array>
#include <cassert>
#include <deque>
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

// Runtime: 6 ms, faster than 58.92% of C++ online submissions for Binary Tree Level Order Traversal.
// Memory Usage: 13.3 MB, less than 12.22% of C++ online submissions for Binary Tree Level Order Traversal.

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
  std::vector<std::vector<int>> levelOrder(TreeNode *root) {
    if (root == nullptr) return {};
    std::vector<std::vector<int>> answer;
    std::queue<TreeNode *> levelNodes;
    levelNodes.push(root);
    while (!levelNodes.empty()) {
      std::queue<TreeNode *> nextLevelNodes;
      answer.push_back(std::vector<int>());
      auto &levelAnswer = answer.back();
      while (!levelNodes.empty()) {
        TreeNode *node = levelNodes.front();
        levelNodes.pop();
        levelAnswer.push_back(node->val);
        if (node->left != nullptr) nextLevelNodes.push(node->left);
        if (node->right != nullptr) nextLevelNodes.push(node->right);
      }

      levelNodes = nextLevelNodes;
    }
    return answer;
  }
};

int main(int argc, char **argv) {}
