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

//  Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
 public:
  // TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
  //   if (root == nullptr || root == p || root == q) return root;
  //   const auto& left = lowestCommonAncestor(root->left, p, q);
  //   const auto& right = lowestCommonAncestor(root->right, p, q);
  //   return left == nullptr ? right : right == nullptr ? left : root;
  // }

  TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
    while ((root->val - p->val) * (root->val - q->val) > 0) {
      root = p->val > root->val ? root = root->right : root = root->left;
    }

    return root;
  }
};

int main(int argc, char** argv) {}
