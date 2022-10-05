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

// Runtime: 35 ms, faster than 49.40% of C++ online submissions for Add One Row to Tree.
// Memory Usage: 25 MB, less than 62.88% of C++ online submissions for Add One Row to Tree.

//  Definition for a binary tree node.
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
  TreeNode *addOneRow(TreeNode *root, int val, int depth) {
    if (root == nullptr) return;
    if (depth > 2) {
      addOneRow(root->left, val, depth - 1);
      addOneRow(root->right, val, depth - 1);
    } else if (depth == 2) {
      TreeNode *newLeft = new TreeNode(val);
      TreeNode *newRight = new TreeNode(val);
      newLeft->left = root->left;
      newRight->right = root->right;
      root->left = newLeft;
      root->right = newRight;
    }

    if (depth == 1) {
      TreeNode *newRoot = new TreeNode(val, root, nullptr);
      root = newRoot;
    }

    return root;
  }
};

int main(int argc, char **argv) {}
