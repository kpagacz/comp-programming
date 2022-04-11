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
    void addSortedElements(std::vector<int>& elements, TreeNode* root) {
      if (root == nullptr)
        return;
      addSortedElements(elements, root->left);
      elements.push_back(root->val);
      addSortedElements(elements, root->right);
    }
    // the invariant is: root1->val is lower than root2 or root2 is nullptr
    void addAllElementsRecursive(std::vector<int>& elements, TreeNode* root1, TreeNode* root2) {
      if (root1 == nullptr && root2 == nullptr)
        return;

      if (root1 == nullptr) {
        addSortedElements(elements, root2);
        return;
      }

      if (root2 == nullptr) {
        addSortedElements(elements, root1);
        return;
      }

      if (root1->val < root2->val) {
        addSortedElements(elements, root1->left);
        elements.push_back(root1->val);
        addAllElementsRecursive(elements, root1->right, root2);
      } else {
        addSortedElements(elements, root2->left);
        elements.push_back(root2->val);
        addAllElementsRecursive(elements, root1, root2->right);
      }
    }
    std::vector<int> getAllElements(TreeNode* root1, TreeNode* root2) {
      std::vector<int> answer;
      addAllElementsRecursive(answer, root1, root2);
      return answer;
    }
};

int main(int argc, char** argv) {}
