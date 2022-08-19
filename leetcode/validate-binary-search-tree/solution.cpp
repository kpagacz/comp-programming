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

// I could have used TreeNode* previous instead of the reference to save some space...

// Runtime: 24 ms, faster than 30.09% of C++ online submissions for Validate Binary Search Tree.
// Memory Usage: 21.7 MB, less than 30.99% of C++ online submissions for Validate Binary Search Tree.

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
  bool isValidBST(TreeNode *root) {
    int64_t previous = -1LL + INT32_MIN;
    return isValidBST(root, previous);
  }
  bool isValidBST(TreeNode *node, int64_t &previous) {
    if (node == nullptr) return true;

    bool leftTreeValid = isValidBST(node->left, previous);
    bool thisNodeValid = node->val > previous;
    previous = node->val;
    bool rightTreeValid = isValidBST(node->right, previous);

    return leftTreeValid && thisNodeValid && rightTreeValid;
  }
};

int main(int argc, char **argv) {}
