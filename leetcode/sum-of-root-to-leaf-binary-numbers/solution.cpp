// link to the problem:https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stdexcept>
#include <string>
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
  int sumUtil(TreeNode *root, int number) {
    number = (number << 1) | root->val;
    if (root->left == nullptr && root->right == nullptr) {
      return  number;
    }
    int sum = 0;
    if (root->left != nullptr) sum += sumUtil(root->left, number);
    if (root->right != nullptr) sum += sumUtil(root->right, number);
    return sum;
  }
  int sumRootToLeaf(TreeNode *root) {
    return sumUtil(root, 0);
  }
};

int main(int argc, char **argv) {}

// What did I learn in this problem?
// if passed digits of a binary number in order I can use << and | to create the number in O(1) space
// instead storing all the digits in the string.
// Although string might be more performant (paradoxically).
