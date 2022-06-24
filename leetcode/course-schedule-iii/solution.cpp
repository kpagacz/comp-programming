// link to the problem: https://leetcode.com/problems/course-schedule-iii/
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

class Solution {
 public:
  int scheduleCourse(std::vector<std::vector<int>>& courses) {
    auto courseDurationCompare = [](const auto& first, const auto& second) { return first[0] < second[0]; };
    std::priority_queue<std::vector<int>, std::vector<std::vector<int>>, decltype(courseDurationCompare)> takenCoursesHeap(
        courseDurationCompare);

    auto courseDeadlineCompare = [](const auto& first, const auto& second) { return first[1] < second[1]; };
    std::sort(courses.begin(), courses.end(), courseDeadlineCompare);

    int32_t totalDuration = 0;
    for(const auto& course : courses) {
      if (totalDuration + course[0] <= course[1]) {
        takenCoursesHeap.push(course);
        totalDuration += course[0];
      } else if(course[0] <= takenCoursesHeap.top()[0]){
        totalDuration -= takenCoursesHeap.top()[0];
        totalDuration += course[0];
        takenCoursesHeap.pop();
        takenCoursesHeap.push(course);
      }
    }

    std::cout << "Top of the heap is: " << takenCoursesHeap.top()[0] << "\n";
    return takenCoursesHeap.size();
  }
};

int main(int argc, char** argv) {}
