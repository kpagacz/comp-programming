// link to the problem: https://leetcode.com/problems/binary-tree-postorder-traversal/
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
  std::vector<int> postorderTraversal(TreeNode *root) {
    std::vector<int> answer;
    postOrderTraversal(root, answer);
    return answer;
  }

 private:
  void postOrderTraversal(TreeNode *node, std::vector<int> &nodes) {
    if (node == nullptr) return;
    if (node->left != nullptr) postOrderTraversal(node->left, nodes);
    if (node->right != nullptr) postOrderTraversal(node->right, nodes);
    nodes.push_back(node->val);
  }
};

int main(int argc, char **argv) {}
