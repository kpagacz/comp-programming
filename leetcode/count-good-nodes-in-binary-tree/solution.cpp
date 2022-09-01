// link to the problem: https://leetcode.com/problems/count-good-nodes-in-binary-tree/
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

// Runtime: 162 ms, faster than 80.51% of C++ online submissions for Count Good Nodes in Binary Tree.
// Memory Usage: 86.4 MB, less than 9.98% of C++ online submissions for Count Good Nodes in Binary Tree.

//  * Definition for a binary tree node.
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
  int goodNodes(TreeNode *root) {
    int answer = 0;
    goodNodes(root, INT32_MIN, answer);
    return answer;
  }

  void goodNodes(TreeNode *node, const int &max, int &answer) {
    if (node == nullptr) return;
    if (node->val >= max) ++answer;
    goodNodes(node->left, std::max(max, node->val), answer);
    goodNodes(node->right, std::max(max, node->val), answer);
  }
};

int main(int argc, char **argv) {}
