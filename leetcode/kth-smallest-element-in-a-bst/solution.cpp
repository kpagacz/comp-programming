// link to the problem: https://leetcode.com/problems/kth-smallest-element-in-a-bst/
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
    int kthSmallest(TreeNode* root, int k) {
      std::stack<TreeNode *> nodes;
      while(!nodes.empty() || root != nullptr) {
        if (root == nullptr) {
          root = nodes.top();
          nodes.pop();
          if (--k == 0) return root->val;
          root = root->right;
        } else {
          nodes.push(root);
          root = root->left;
        }
      }
      return -1;
    }
};

int main(int argc, char** argv) {}
