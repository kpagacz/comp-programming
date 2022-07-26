// link to the problem: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
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
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

// Runtime: 10 ms, faster than 99.13% of C++ online submissions for Lowest Common Ancestor of a Binary Tree.
// Memory Usage: 14.2 MB, less than 85.91% of C++ online submissions for Lowest Common Ancestor of a Binary Tree.

class Solution {
 public:
  TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
    if (root == nullptr || root == p || root == q) return root;
    const auto& left = lowestCommonAncestor(root->left, p, q);
    const auto& right = lowestCommonAncestor(root->right, p, q);
    return left == nullptr ? right : right == nullptr ? left : root;
  }
};

int main(int argc, char** argv) {
  TreeNode* root = new TreeNode(3);
  root->left = new TreeNode(5);
  root->left->left = new TreeNode(6);
  root->left->right = new TreeNode(2);
  root->left->right->left = new TreeNode(7);
  root->left->right->right = new TreeNode(4);
  root->right = new TreeNode(1);
  Solution s;
  const auto& found = s.lowestCommonAncestor(root, root->left, root->right);
  if (found != nullptr) std::cout << "Found: " << found->val << '\n';
}
