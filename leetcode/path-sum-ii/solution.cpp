// link to the problem: https://leetcode.com/problems/path-sum-ii/
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

// Runtime: 13 ms, faster than 77.48% of C++ online submissions for Path Sum II.
// Memory Usage: 20.1 MB, less than 49.33% of C++ online submissions for Path Sum II.

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
  std::vector<std::vector<int>> pathSum(TreeNode *root, int targetSum) {
    if (root == nullptr) return {};
    std::vector<std::vector<int>> answer;
    this->targetSum = targetSum;
    std::vector<int> currentPath;
    pathSum(answer, currentPath, root, 0);
    return answer;
  }

  void pathSum(std::vector<std::vector<int>> &paths, std::vector<int> &currentPath, TreeNode *node, const int &sum) {
    int newSum = sum + node->val;
    currentPath.push_back(node->val);
    if (node->left == nullptr && node->right == nullptr) {
      if (newSum == targetSum) paths.push_back(currentPath);
      currentPath.pop_back();
      return;
    }
    if (node->left) pathSum(paths, currentPath, node->left, newSum);
    if (node->right) pathSum(paths, currentPath, node->right, newSum);
    currentPath.pop_back();
  }

  int targetSum;
};

int main(int argc, char **argv) {}
