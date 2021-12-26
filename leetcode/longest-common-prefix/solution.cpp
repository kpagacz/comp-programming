// link to the problem: https://leetcode.com/problems/longest-common-prefix/submissions/

// What have I learned?
// * watch out for conditions in long boolean expressions (make sure you compare the right thing!)
// * remember to break early if needed
#include <algorithm>
#include <string>
#include <vector>

class Solution {
 public:
  std::string longestCommonPrefix(std::vector<std::string>& strs) {
    int longest = 0;
    for (int i = 0; i < 201; i++) {
      bool same = true;
      for (auto j = 0; j < strs.size(); j++) {
        same = same && i < strs[j].size() && strs[j][i] == strs[0][i];
      }
      if (same) longest++; else break;
    }

    return longest ? strs[0].substr(0, longest) : "";
  }
};
