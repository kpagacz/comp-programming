// link to the problem: https://leetcode.com/problems/count-complete-tree-nodes/
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
  int countNodes(TreeNode *root) {
    if (root == nullptr) return 0;
    return (1 << (height(root) - 1)) - 1 + lastLevelNodes(root, 0, height(root));
  }
  int height(TreeNode *root) {
    if (root == nullptr) return 0;
    return 1 + std::max(height(root->right), height(root->left));
  }

  int lastLevelNodes(TreeNode *root, const int &currentLevel, const int &lastLevel) {
    if (root && currentLevel == lastLevel) return 1;
    if (root == nullptr) return 0;
    int nodesCount = 0;
    if (root->right) nodesCount += lastLevelNodes(root->right, currentLevel + 1, lastLevel);
    if (root->left) nodesCount += lastLevelNodes(root->left, currentLevel + 1, lastLevel);
    return nodesCount;
  }
};

// class Solution {
//  public:
//   int countNodes(TreeNode *root, int l = 1, int r = 1) {
//     if (!root) return 0;

//     TreeNode *left = root, *right = root;
//     while (left = left->left) ++l;
//     while (right = right->right) ++r;

//     if (l == r) return (1 << l) - 1;

//     return 1 + countNodes(root->left) + countNodes(root->right);
//   }
// };

int main(int argc, char **argv) {}
