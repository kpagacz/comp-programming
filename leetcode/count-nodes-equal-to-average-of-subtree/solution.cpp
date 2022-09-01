// link to the problem:
// https://leetcode.com/contest/weekly-contest-292/problems/count-nodes-equal-to-average-of-subtree/
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
  int averageOfSubtree(TreeNode *root) {
    int answer = 0;
    averageOfSubtree(root, answer);
    return answer;
  }

  std::pair<int, int> averageOfSubtree(TreeNode *node, int &answer) {  // sum, no of nodes
    if (node == nullptr) return {0, 0};
    auto [leftSum, leftNodes] = averageOfSubtree(node->left, answer);
    auto [rightSum, rightNodes] = averageOfSubtree(node->right, answer);

    if ((leftSum + rightSum + node->val) / (leftNodes + rightNodes + 1) == node->val) ++answer;

    return {leftSum + rightSum + node->val, leftNodes + rightNodes + 1};
  }
};

int main(int argc, char **argv) {}
