// link to the problem: https://leetcode.com/problems/binary-tree-inorder-traversal/
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
  void inorderRecursive(std::vector<int> &answer, TreeNode *root) {
    if (root == nullptr) return;
    inorderRecursive(answer, root->left);
    answer.push_back(root->val);
    inorderRecursive(answer, root->right);
  }
  std::vector<int> inorderTraversal(TreeNode *root) {
    std::vector<int> answer;
    inorderRecursive(answer, root);
    return answer;
  }
};

int main(int argc, char **argv) {}
