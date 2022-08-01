// link to the problem: https://leetcode.com/problems/find-closest-node-to-given-two-nodes/
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
  int closestMeetingNode(std::vector<int>& edges, int node1, int node2) {
    std::vector<int> node1Distances(edges.size(), -1);
    std::vector<int> node2Distances(edges.size(), -1);

    int distance = 0;
    int currentIndex = node1;
    while (node1Distances[currentIndex] == -1) {
      node1Distances[currentIndex] = distance++;
      if (edges[currentIndex] != -1)
        currentIndex = edges[currentIndex];
      else
        break;
    }

    distance = 0;
    currentIndex = node2;
    while (node2Distances[currentIndex] == -1) {
      node2Distances[currentIndex] = distance++;
      if (edges[currentIndex] != -1)
        currentIndex = edges[currentIndex];
      else
        break;
    }

    int distanceMaximum = INT32_MAX;
    int maximumDistanceNodeIndex = -1;

    for (int i = 0; i < edges.size(); ++i) {
      if (node1Distances[i] == -1 || node2Distances[i] == -1) continue;
      const auto& max = std::max(node1Distances[i], node2Distances[i]);
      if (max < distanceMaximum) {
        distanceMaximum = max;
        maximumDistanceNodeIndex = i;
      }
    }

    return maximumDistanceNodeIndex;
  }
};

int main(int argc, char** argv) {}
