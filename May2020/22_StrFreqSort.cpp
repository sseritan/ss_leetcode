#include <map>
#include <string>
#include <utility> // std::pair
#include <vector>

bool valComp(const std::pair<char, int> &a,
  const std::pair<char, int> &b)
{
  return a.second > b.second;
}

class Solution {
public:
  std::string frequencySort(std::string s) {
    std::map<char, int> counts;

    // Count frequency (O(N) runtime and memory)
    for (int i = 0; i < s.size(); ++i) {
      char c = s[i];
      if (!counts.count(c)) {
        counts[c] = 1;
      } else {
        counts[c] += 1;
      }
    }

    // Cast to vector to use std::sort on values (O(N) runtime and memory)
    std::vector<std::pair<char, int> > pairs;
    for (auto it = counts.begin(); it != counts.end(); ++it) {
      pairs.push_back(*it);
    }

    // Sort on value (using std::sort so O(NlogN)?)
    std::sort(pairs.begin(), pairs.end(), valComp);

    // Build new string (O(N) runtime and memory)
    std::string sorted;
    for (auto it = pairs.begin(); it != pairs.end(); ++it) {
      sorted.append(it->second, it->first);
    }

    return sorted;
  }
};

int main(int argc, char **argv)
{
  Solution sol;

  if (argc != 2) {
    printf("Usage: %s <string to sort>\n", argv[0]);
  }

  printf("%s\n", sol.frequencySort(argv[1]).c_str());

  return 0;
}