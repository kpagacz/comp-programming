// link to the problem: https://leetcode.com/problems/binary-tree-preorder-traversal/
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
  std::vector<int> preorderTraversal(TreeNode *root) {
    std::vector<int> answer;
    preOrderTraversal(root, answer);
    return answer;
  }

 private:
  void preOrderTraversal(TreeNode *node, std::vector<int> &nodes) {
    if (node == nullptr) return;
    nodes.push_back(node->val);
    preOrderTraversal(node->left, nodes);
    preOrderTraversal(node->right, nodes);
  }
};

int main(int argc, char **argv) {}
