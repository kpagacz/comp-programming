// link to the problem: https://leetcode.com/problems/all-elements-in-two-binary-search-trees/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
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
    void inOrderTraversal(TreeNode* root, std::vector<int>& arr) {
      if (root) {
        inOrderTraversal(root->left, arr);
        arr.push_back(root->val);
        inOrderTraversal(root->right);
      }
    }
    void mergeTwoArrays(const std::vector<int>& first, const std::vector<int>& second, const int ind1, const int ind2, std::vector<int> &arr) {
      if (first.size() == ind1 && second.size() == ind2)
        return;
      if (first.size() == ind1) {
        arr.push_back(second[ind2]);
        mergeTwoArrays(first, second, ind1, ind2 + 1, arr);
        return;
      }
      if (second.size() == ind2) {
        arr.push_back(first[ind1]);
        mergeTwoArrays(first, second, ind1 + 1, ind2, arr);
        return;
      }
      if (first[ind1] < second[ind2]) {
        arr.push_back(first[ind1]);
        mergeTwoArrays(first, second, ind1 + 1, ind2, arr);
      } else {
        arr.push_back(second[ind2]);
        mergeTwoArrays(first, second, ind1, ind2 + 1, arr);
      }
    }
    std::vector<int> getAllElements(TreeNode* root1, TreeNode* root2) {
      std::vector<int> first, second;
      inOrderTraversal(root1, first);
      inOrderTraversal(root2, second);
      std::vector<int> answer;
      mergeTwoArrays(first, second, 0, 0, answer);
      return answer;
    }
};

int main(int argc, char** argv) {}
