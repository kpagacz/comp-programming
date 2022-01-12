// link to the problem: https://leetcode.com/problems/insert-into-a-binary-search-tree/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stdexcept>
#include <string>
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
  TreeNode *insertIntoBST(TreeNode *root, int val) {
    if (root == nullptr) return new TreeNode(val);
    TreeNode *par = nullptr, *it = root;
    while (it != nullptr) {
      par = it;
      if (val < it->val) it = it->left;
      else it = it->right;
    }
    if (val < par->val) par->left = new TreeNode(val);
    else par->right = new TreeNode(val);
    return root;
  }

  TreeNode *insertIntoBSTRec(TreeNode *root, int val) {
    if (root == nullptr) return new TreeNode(val);
    if (val < root->val) root->left = insertIntoBST(root->left, val);
    else root->right = insertIntoBST(root->right, val);
    return root;
  }
};

int main(int argc, char **argv) {}
