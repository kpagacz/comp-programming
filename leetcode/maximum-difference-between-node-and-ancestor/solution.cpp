// link to the problem: https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/
#include<iostream>
#include<vector>
#include<array>
#include<string>
#include<algorithm>
#include<numeric>
#include<sstream>
#include<iterator>


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
 int max_diff;
 void maxAncestorDiffRec(TreeNode* it, int max, int min) {
   if (it == nullptr) return;
   if (it->val < min) {
     min = it->val;
   }
   if (it->val > max){
    max = it->val;
   }
   if (max - min > max_diff) max_diff = max - min;
   maxAncestorDiffRec(it->left, max, min);
   maxAncestorDiffRec(it->right, max, min);
 }
 int maxAncestorDiff(TreeNode *root) {
   maxAncestorDiffRec(root, root->val, root->val);
   return max_diff;
 }
};
