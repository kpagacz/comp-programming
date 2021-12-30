// link to the problem:
// https://leetcode.com/problems/populating-next-right-pointers-in-each-node/
#include <algorithm>
#include <array>
#include <iostream>
#include <iterator>
#include <numeric>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

// Definition for a Node.
class Node {
 public:
  int val;
  Node* left;
  Node* right;
  Node* next;

  Node() : val(0), left(NULL), right(NULL), next(NULL) {}

  Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}

  Node(int _val, Node* _left, Node* _right, Node* _next)
      : val(_val), left(_left), right(_right), next(_next) {}

  void print() {
    Node* it = this;
    while(it != nullptr) {
      std::cout << it->val << " ";
      it = it->next;
    }
    std::cout << "#";
    if (left != nullptr) left->print();
  }
};

class Solution {
 public:
  std::unordered_map<Node*, Node*> parents;

  void find_parents(Node* root) {
    if (root == nullptr) return;
    if (root->left != nullptr) {
      parents.insert({root->left, root});
      parents.insert({root->right, root});
    }
    find_parents(root->left);
    find_parents(root->right);
  }

  Node* find_next(Node* root) {
    Node *parent, *it = root;
    if (parents.find(root) == parents.end()) return nullptr;
    parent = parents[it];

    int up = 1;
    while (it != parent->left) {
      it = parent;
      if (parents.find(parent) == parents.end()) return nullptr;
      parent = parents[parent];
      up++;
    }

    it = parent->right;
    while (up) {
      parent = it;
      it = it->left;
      up--;
    }
    return parent;
  }

  void recursive_connect(Node* root) {
    if (root == nullptr) return;
    root->next = find_next(root);
    recursive_connect(root->left);
    recursive_connect(root->right);
  }

  Node* connect(Node* root) {
    find_parents(root);
    recursive_connect(root);
    return (root);
  }
};

int main() {
  Node* root = new Node(1);
  root->left = new Node(2);
  root->right = new Node(3);
  root->left->left = new Node(4);
  root->left->right = new Node(5);
  root->right->left = new Node(6);
  root->right->right = new Node(7);
  Solution s;
  root = s.connect(root);
  root->print();
}
