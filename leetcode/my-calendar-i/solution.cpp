// link to the problem: https://leetcode.com/problems/my-calendar-i/
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

// Runtime: 117 ms, faster than 81.47% of C++ online submissions for My Calendar I.
// Memory Usage: 38.9 MB, less than 48.99% of C++ online submissions for My Calendar I.

class MyCalendar {
 public:
  MyCalendar() {}

  bool book(int start, int end) {
    auto lowerBound = bookings.lower_bound({start, end});

    bool canBook = true;
    if (lowerBound != bookings.begin() && start < (std::next(lowerBound, -1))->second) return false;
    if (lowerBound != bookings.end() && end > lowerBound->first) return false;

    bookings.insert({start, end});
    return true;
  }

 private:
  std::set<std::pair<int, int>> bookings;
};

/**
 * Your MyCalendar object will be instantiated and called as such:
 * MyCalendar* obj = new MyCalendar();
 * bool param_1 = obj->book(start,end);
 */

int main(int argc, char** argv) {}
