// link to the problem: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
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

// I hate everything about this solution. From its complexity to runtime.
// I though it could at least be fast, but it is super slow as well. What was the point?
// Runtime: 38 ms, faster than 13.78% of C++ online submissions for Lowest Common Ancestor of a Binary Tree.
// Memory Usage: 23.5 MB, less than 6.15% of C++ online submissions for Lowest Common Ancestor of a Binary Tree.

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class DisjointSet {
 public:
  DisjointSet(std::size_t size) {
    parent.resize(size);
    std::iota(parent.begin(), parent.end(), 0);
    rank.resize(size, 0);
  }

  DisjointSet() : DisjointSet(0) {}

  void makeSet() {
    parent.push_back(parent.size());
    rank.push_back(0);
  }

  int findSet(int element) {
    if (element >= parent.size()) return -1;
    if (parent[element] == element) return element;
    return parent[element] = findSet(parent[element]);
  }

  void unionSet(int element1, int element2) {
    if (element1 >= parent.size() || element2 >= parent.size()) return;
    const auto& root1 = findSet(element1);
    const auto& root2 = findSet(element2);

    if (root1 == root2) return;

    if (rank[root1] > rank[root2]) {
      parent[root2] = root1;
    } else {
      parent[root1] = root2;
      if (rank[root1] == rank[root2]) ++rank[root2];
    }
  }

 private:
  std::vector<int> parent, rank;
};

class Solution {
 public:
  TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
    if (root == nullptr) return nullptr;
    DisjointSet unionFind;
    std::vector<TreeNode*> ancestor;
    std::unordered_map<TreeNode*, int> nodeMap;
    std::unordered_set<int> colored;
    const auto& lca = tarjanOLCA(root, {p->val, q->val}, unionFind, ancestor, colored, nodeMap);
    if (lca == nullptr)
      return root;
    else
      return lca;
  }

 private:
  TreeNode* tarjanOLCA(TreeNode* node, const std::set<int>& pairs, DisjointSet& unionFind,
                       std::vector<TreeNode*>& ancestors, std::unordered_set<int>& colored,
                       std::unordered_map<TreeNode*, int>& nodeMap) {
    if (node == nullptr) return nullptr;
    unionFind.makeSet();
    nodeMap[node] = nodeMap.size();
    ancestors.push_back(node);

    for (const auto& child : {node->left, node->right}) {
      if (child == nullptr) continue;
      const auto& lca = tarjanOLCA(child, pairs, unionFind, ancestors, colored, nodeMap);
      if (lca != nullptr) return lca;
      unionFind.unionSet(nodeMap[node], nodeMap[child]);
      ancestors[unionFind.findSet(nodeMap[node])] = node;
    }
    colored.insert(node->val);

    if (pairs.find(node->val) != pairs.end()) {
      for (const auto& value : pairs) {
        if (value != node->val && colored.find(value) != colored.end()) {
          const auto& foundOther = std::find_if(nodeMap.begin(), nodeMap.end(), [&](const auto& pair) {
                                     return pair.first->val == value;
                                   })->first;
          const auto& lca = ancestors[unionFind.findSet(nodeMap[foundOther])];
          return lca;
        }
      }
    }
    return nullptr;
  }
};

int main(int argc, char** argv) {
  TreeNode* root = new TreeNode(3);
  root->left = new TreeNode(5);
  // root->left->left = new TreeNode(6);
  // root->left->right = new TreeNode(2);
  // root->left->right->left = new TreeNode(7);
  // root->left->right->right = new TreeNode(4);
  root->right = new TreeNode(1);
  Solution s;
  const auto& found = s.lowestCommonAncestor(root, root->left, root->right);
  if (found != nullptr) std::cout << "Found: " << found->val << '\n';
}
