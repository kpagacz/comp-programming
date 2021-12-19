#include <iostream>
#include <string>
#include <algorithm>
#include <numeric>
#include<vector>

struct Node
{
  Node *left, *right, *parent;
  int value;
  Node(int _value)
  {
    value = _value;
    left = right = parent = nullptr;
  }

  Node()
  {
    value = -1;
    left = right = parent = nullptr;
  }

  Node(Node *p, int _value)
  {
    value = _value;
    parent = p;
    left = right = nullptr;
  }

  void print()
  {
    if (value != -1)
    {
      std::cout << value;
    }
    else
    {
      std::cout << '[';
      left->print();
      std::cout << ',';
      right->print();
      std::cout << ']';
    }
  }

  ~Node()
  {
    delete left;
    delete right;
  }
};

bool is_left_child(Node *p, Node *c)
{
  return p->left == c;
}

Node *find_max_node(Node *n)
{
  if (n->right == nullptr)
    return n;
  else
    return find_max_node(n->right);
}

Node *find_min_node(Node *n)
{
  if (n->left == nullptr)
    return n;
  else
    return find_min_node(n->left);
}

Node *find_left_leaf(Node *n)
{
  Node *p_it = n->parent, *it = n;
  while (p_it != nullptr && is_left_child(p_it, n))
  {
    n = p_it;
    p_it = p_it->parent;
  }

  if (p_it == nullptr)
    return nullptr;
  return find_max_node(p_it->left);
}

Node *find_right_leaf(Node *n)
{
  Node *p_it = n->parent, *it = n;
  while (p_it != nullptr && !is_left_child(p_it, n))
  {
    n = p_it;
    p_it = p_it->parent;
  }
  if (p_it == nullptr)
    return nullptr;
  return find_min_node(p_it->right);
}

void explode_pair(Node *n)
{
  Node *left = find_left_leaf(n);
  Node *right = find_right_leaf(n);
  if (left != nullptr)
    left->value += n->left->value;
  if (right != nullptr)
    right->value += n->right->value;
  n->left = n->right = nullptr;
  n->value = 0;
}

int find_middle_comma(const std::string &in)
{
  int left_brackets = 0;
  for (int i = 0; i < in.size(); i++)
  {
    switch (in[i])
    {
    case '[':
      left_brackets++;
      break;
    case ']':
      left_brackets--;
      break;
    case ',':
      if (left_brackets == 1)
        return i;
    }
  }
  return -1;
}

Node* find_splittable(Node *root)
{
  if (root == nullptr)
    return nullptr;
  if (root->value > 9)
    return root;
  auto left_splittable = find_splittable(root->left);
  if (left_splittable != nullptr)
    return left_splittable;
  return find_splittable(root->right);
}

Node*  find_explodable(Node *root, int depth)
{
  if (root->value == -1)
  {
    if (depth >= 5)
      return root;
    auto left_explodable = find_explodable(root->left, depth + 1);
    if (left_explodable != nullptr)
      return left_explodable;
    return find_explodable(root->right, depth + 1);
  }
  else
  {
    return nullptr;
  }
}

void split(Node *root)
{
    root->left = new Node(root, root->value / 2);
    root->right = new Node(root, root->value / 2 + root->value % 2);
    root->value = -1;
}

Node *parse_node(const std::string &in)
{
  Node *new_node = new Node();

  if (in[0] != '[')
  {
    new_node->value = std::stoi(in, 0, 10);
  }
  else
  {
    int comma_pos = find_middle_comma(in);
    std::string left = in.substr(1, comma_pos - 1);
    std::string right = in.substr(comma_pos + 1, in.size() - comma_pos - 2);
    new_node->left = parse_node(left);
    new_node->left->parent = new_node;
    new_node->right = parse_node(right);
    new_node->right->parent = new_node;
  }

  return new_node;
}

int magnitude(Node *root)
{
  if (root == nullptr)
    return 0;
  if (root->value != -1)
    return root->value;
  return 3 * magnitude(root->left) + 2 * magnitude(root->right);
}

Node *add(Node *first, Node *second)
{
  Node *new_root = new Node();
  new_root->left = first;
  new_root->right = second;
  first->parent = new_root;
  second->parent = new_root;

  while (true)
  {
    auto explodable = find_explodable(new_root, 1);
    if (explodable != nullptr) {
      explode_pair(explodable);
      continue;
    }
    auto splittable = find_splittable(new_root);
    if (splittable != nullptr) {
      split(splittable);
      continue;
    }
    break;
  }

  return new_root;
}

Node* clone(const Node* root) {
  Node *new_root = new Node();
  if (root == nullptr)
    return nullptr;
  if (root->value != -1) {
    new_root->value = root->value;
    return new_root;
  } else {
    new_root->left = clone(root->left);
    new_root->left->parent = new_root;
    new_root->right = clone(root->right);
    new_root->right->parent = new_root;
    return new_root;
  }
}

int main(int argc, char **argv)
{
  std::string line;
  std::vector<Node *> numbers;

  while(std::cin >> line)
    numbers.push_back(parse_node(line));

  int max_magnitude = 0;
  for (int i = 0; i < numbers.size(); i++) {
    for (int j = i + 1; j < numbers.size(); j++) {
      auto n1 = clone(numbers[i]);
      auto n2 = clone(numbers[j]);
      if (magnitude(add(n1, n2)) > max_magnitude)
        max_magnitude = magnitude(add(n1, n2));
      n1 = clone(numbers[i]);
      n2 = clone(numbers[j]);
      if(magnitude(add(n2, n1)) > max_magnitude)
        max_magnitude = magnitude(add(n2, n1));
    }
  }

  std::cout << "Max magnitude is: " << max_magnitude << '\n';
}
