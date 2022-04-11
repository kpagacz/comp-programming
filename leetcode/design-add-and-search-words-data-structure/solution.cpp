// link to the problem: https://leetcode.com/problems/design-add-and-search-words-data-structure/
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

class WordDictionary {
public:
  struct TrieNode {
    char letter;
    std::unordered_map<char, TrieNode *> children;
    bool word_end = false;
    TrieNode() = default;
    TrieNode(char c) {
      letter = c;
    }
  };

  TrieNode *root;
    WordDictionary() {
      root = new TrieNode();
    }

    void addWord(std::string word) {
      TrieNode *it = root;
      for(const auto& c : word) {
        if (it->children.find(c) == it->children.end()) {
          TrieNode *new_letter = new TrieNode(c);
          it->children.insert({c, new_letter});
        }
        it = it->children[c];
      }
      it->word_end = true;
    }

    bool searchRecursive(const std::string& word, const int index, TrieNode* it) {
      if (index == word.size())
        return it->word_end;

      bool answer = false;
      if (word[index] == '.') for(const auto& child : it->children)
          answer |= searchRecursive(word, index + 1, child.second);
      else if (it->children.find(word[index]) != it->children.end())
        answer |= searchRecursive(word, index + 1, it->children[word[index]]);

      return answer;
    }

    bool search(std::string word) {
      return searchRecursive(word, 0, root);
    }
};

void printTrie(WordDictionary::TrieNode* root, int depth) {
  for (auto i{0}; i < depth; i++)
    std::cout << " ";
  std::cout << "Letter: " << root->letter << " children: ";
  for(const auto& child : root->children)
    std::cout << child.first << " ";
  std::cout << "Word ends here: " << std::boolalpha << root->word_end;
  std::cout << "\n";
  for(const auto& child : root->children)
    printTrie(child.second, depth + 1);
}


int main(int argc, char** argv) {
  WordDictionary *dict = new WordDictionary();

  dict->addWord("ran");
  dict->addWord("rune");
  dict->addWord("runner");
  dict->addWord("runs");
  dict->addWord("add");
  dict->addWord("adds");
  dict->addWord("adder");
  dict->addWord("addee");


  WordDictionary::TrieNode *it = dict->root;
  printTrie(it, 0);

  std::cout << std::boolalpha << dict->search("...s") << " " << dict->search("");
}
