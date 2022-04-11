// link to the problem: https://leetcode.com/problems/symmetric-tree/
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
  bool isSymmetric(TreeNode *root) {
    if (root == nullptr) return true;
    std::vector<TreeNode *> left, right;
    if (root->left != nullptr) left.push_back(root->left);
    if (root->right != nullptr) right.push_back(root->right);
    while (!left.empty() && !right.empty()) {
      if (left.size() != right.size()) return false;
      std::vector<TreeNode *> new_left, new_right;
      bool equal = std::equal(left.begin(), left.end(), right.rbegin(), right.rend(), [](const auto &l, const auto &r) {
        if (l == nullptr && r != nullptr || l != nullptr && r == nullptr) return false;
        if (l == nullptr && r == nullptr) return true;
        return l->val == r->val;
      });
      if (!equal) return false;
      for (const auto &node : left)
        if (node != nullptr) {
          new_left.push_back(node->left);
          new_left.push_back(node->right);
        }
      for (const auto &node : right)
        if (node != nullptr) {
          new_right.push_back(node->left);
          new_right.push_back(node->right);
        }
      left = new_left;
      right = new_right;
    }
    return left.size() == right.size();
  }

  bool isSymmetricRec(TreeNode *root) { return root == nullptr || recHelper(root->left, root->right); }

  bool recHelper(TreeNode *left, TreeNode *right) {
    if (left == nullptr || right == nullptr) return left == right;
    if (left->val != right->val) return false;
    return recHelper(left->left, right->right) && recHelper(left->right, right->left);
  }
};

int main(int argc, char **argv) {}
