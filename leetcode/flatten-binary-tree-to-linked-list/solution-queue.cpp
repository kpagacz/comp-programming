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
  void flatten(TreeNode *root) {
    std::queue<TreeNode *> nodes;
    flatten(root, nodes);
    while (nodes.size() > 1) {
      auto front = nodes.front();
      nodes.pop();
      front->right = nodes.front();
      front->left = nullptr;
    }
    if (!nodes.empty()) {
      nodes.front()->right = nullptr;
      nodes.front()->left = nullptr;
    }
  }

 private:
  void flatten(TreeNode *node, std::queue<TreeNode *> &nodes) {
    if (node == nullptr) return;
    nodes.push(node);
    flatten(node->left, nodes);
    flatten(node->right, nodes);
  }
};

int main(int argc, char **argv) {}
