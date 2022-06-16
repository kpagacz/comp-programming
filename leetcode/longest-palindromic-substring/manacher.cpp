#include <algorithm>
#include <iostream>
#include <iterator>
#include <sstream>
#include <string>

class Solution {
 public:
  std::string longestPalindrome(std::string s) {
    std::stringstream os;
    os << "|";
    std::copy(s.begin(), s.end(), std::ostream_iterator<char>(os, "|"));
    s = os.str();

    int *palindrome_radii = new int[s.size()];

    int center{0}, radius{0};
    while (center < s.size()) {
      // At the start of the loop, Radius is already set to a lower-bound for the longest radius.
      // In the first iteration, Radius is 0, but it can be higher.

      // Determine the longest palindrome starting at Center-Radius and going to Center+Radius
      while (center - radius - 1 >= 0 && center + radius + 1 < s.size() &&
             s[center - radius - 1] == s[center + radius + 1])
        ++radius;

      palindrome_radii[center] = radius;

      // Below, Center is incremented.
      // If any precomputed values can be reused, they are.
      // Also, Radius may be set to a value greater than 0
      int old_center{center}, old_radius{radius};
      ++center;
      // Radius' default value will be 0, if we reach the end of the following loop.
      radius = 0;
      while (center <= old_center + old_radius) {
        // Because Center lies inside the old palindrome and every character inside
        // a palindrome has a "mirrored" character reflected across its center, we
        // can use the data that was precomputed for the Center's mirrored point.
        int mirrored_center = old_center - (center - old_center);
        int max_mirrored_radius = old_center + old_radius - center;

        if (palindrome_radii[mirrored_center] != max_mirrored_radius) {
          palindrome_radii[center] = std::min(palindrome_radii[mirrored_center], max_mirrored_radius);
          ++center;
        } else {  // palindrome_radii[mirrored_center] == max_mirrored_radius
          radius = max_mirrored_radius;
          break;
        }
      }
    }

    int longest_palindrome_radii{0}, longest_palindrome_center{0};
    for (int i = 0; i < s.size(); i++) {
      if (palindrome_radii[i] > longest_palindrome_radii) {
        longest_palindrome_center = i;
        longest_palindrome_radii = palindrome_radii[i];
      }
    }

    std::string answer =
        s.substr(longest_palindrome_center - longest_palindrome_radii, 2 * longest_palindrome_radii + 1);
    answer.erase(std::remove(answer.begin(), answer.end(), '|'), answer.end());
    delete[] palindrome_radii;
    return answer;
  }
};
