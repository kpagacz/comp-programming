// link to the problem: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
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
  void flatten(TreeNode *root) { flattenWithTail(root); }

 private:
  TreeNode *flattenWithTail(TreeNode *node) {
    if (node == nullptr) return nullptr;
    auto leftEnd = flattenWithTail(node->left);
    auto rightEnd = flattenWithTail(node->right);
    if (leftEnd != nullptr) {
      leftEnd->right = node->right;
      node->right = node->left;
    }
    node->left = nullptr;
    return rightEnd != nullptr ? rightEnd : leftEnd != nullptr ? leftEnd : node;
  }
};

int main(int argc, char **argv) {
  TreeNode *root = new TreeNode(1);
  // root->left = new TreeNode(2);
  // root->left->left = new TreeNode(3);
  // root->left->right = new TreeNode(4);
  root->right = new TreeNode(5);
  root->right->right = new TreeNode(6);

  Solution s;
  s.flatten(root);
  while (root != nullptr) {
    std::cout << root->val << " ";
    root = root->right;
  }
}
