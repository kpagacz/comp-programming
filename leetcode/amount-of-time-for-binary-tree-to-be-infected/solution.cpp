// link to the problem: https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/
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

// Runtime: 1250 ms, faster than 14.29% of C++ online submissions for Amount of Time for Binary Tree to Be Infected.
// Memory Usage: 629.3 MB, less than 14.29% of C++ online submissions for Amount of Time for Binary Tree to Be Infected.

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
  int amountOfTime(TreeNode *root, int start) {
    std::unordered_map<int, std::vector<int>> graph = toUndirectedGraph(root);
    std::unordered_set<int> infectedNodes;
    std::queue<int> freshlyInfected;
    freshlyInfected.push(start);
    int answer = -1;
    int oldSize = infectedNodes.size();
    while (infectedNodes.size() != graph.size()) {
      answer++;
      std::queue<int> newInfected;
      while (!freshlyInfected.empty()) {
        auto front = freshlyInfected.front();
        freshlyInfected.pop();
        if (infectedNodes.count(front) > 0) continue;
        infectedNodes.insert(front);
        for (const auto &neighbour : graph[front]) newInfected.push(neighbour);
      }
      freshlyInfected = newInfected;
    }
    return answer;
  }

  std::unordered_map<int, std::vector<int>> toUndirectedGraph(TreeNode *root) {
    std::unordered_map<int, std::vector<int>> graph;
    addToGraph(nullptr, root, graph);
    return graph;
  }

  void addToGraph(TreeNode *parent, TreeNode *node, std::unordered_map<int, std::vector<int>> &graph) {
    if (node == nullptr) return;
    std::vector<int> neighbours;
    for (const auto &neighbour : {parent, node->left, node->right})
      if (neighbour != nullptr) neighbours.push_back(neighbour->val);
    graph[node->val] = neighbours;
    addToGraph(node, node->left, graph);
    addToGraph(node, node->right, graph);
  }
};

int main(int argc, char **argv) {
  Solution s;
  TreeNode *root = new TreeNode(1);
  root->left = new TreeNode(5);
  root->left->right = new TreeNode(4);
  root->right = new TreeNode(3);

  std::cout << s.amountOfTime(root, 3) << '\n';
}
