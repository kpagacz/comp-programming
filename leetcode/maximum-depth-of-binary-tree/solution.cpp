// link to the problem: https://leetcode.com/problems/maximum-depth-of-binary-tree/
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
    int recursiveDepth(TreeNode* node, int depth) {
      if (node == nullptr) return depth - 1;
      return std::max(recursiveDepth(node->left, depth + 1), recursiveDepth(node->right, depth + 1));
    }
    int maxDepth(TreeNode *root) { return recursiveDepth(root, 1); }
};

int main(int argc, char** argv) {}
