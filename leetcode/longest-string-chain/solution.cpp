// link to the problem: https://leetcode.com/problems/longest-string-chain/
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

struct ChainWord {
  std::string word;
  int steps;
  ChainWord(std::string word) {
    this->word = word;
    steps = 1;
  }
};

class Solution {
 public:
  int longestStrChain(std::vector<std::string>& words) {
    std::unordered_map<std::string, bool> done;
    for (auto w : words) done.insert({w, false});

    auto chain_word_cmpr = [](auto first, auto second) {
      if (first->word.size() != second->word.size()) return first->word.size() < second->word.size();
      return first->steps < second->steps;
    };
    std::priority_queue<ChainWord*, std::vector<ChainWord*>, decltype(chain_word_cmpr)> queue(chain_word_cmpr);
    for (auto w : words) queue.push(new ChainWord(w));

    int longest_chain = 1;
    while (!queue.empty()) {
      auto chain_word = queue.top();
      queue.pop();

      // std::cout << "Word: " << chain_word->word << " steps: " << chain_word->steps << '\n';
      if (done[chain_word->word]) continue;
      longest_chain = std::max(longest_chain, chain_word->steps);

      for (int i = 0; i < chain_word->word.size(); i++) {
        std::string new_word(chain_word->word);
        new_word.erase(i, 1);
        // std::cout << "New word: " << new_word << '\n';
        if (done.count(new_word) > 0) {
          auto new_chain_word = new ChainWord(new_word);
          new_chain_word->steps = chain_word->steps + 1;
          queue.push(new_chain_word);
        }
      }

      done[chain_word->word] = true;
    }
    return longest_chain;
  }
};
