#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// The API isBadVersion is defined for you.
// bool isBadVersion(int version);
int badVersion;
int apiCalls;
bool isBadVersion(int version)
{
  apiCalls++;
  //if (apiCalls > 10) {
  //  exit(1);
  //}
  return version >= badVersion;
}

class Solution {
public:
  int firstBadVersion(int n) {
    // Classic bisection, O(logN) time and O(1) space
    int lower = 1;
    int upper = n;
    int pivot = n / 2;

    // Store value for lower/upper for base case
    // While technically less ideal to call two off the bat
    // it catches worst cases right away and doesn't contribute to scaling
    bool lval = isBadVersion(lower);
    if (lval) {
      printf("First entry is bad\n");
      return lower;
    }
    bool uval = isBadVersion(upper);
    if (!uval) {
      printf("Last entry is good\n");
      return upper;
    }
    if (n == 2) {
      return (uval ? 2 : 1);
    }

    bool pval;
    while (upper - lower > 1) {
      pivot = (upper - lower) / 2 + lower; // Avoid int overflow
      //printf("Lower: %d Upper: %d Pivot: %d => ", lower, upper, pivot);

      pval = isBadVersion(pivot);
      if (pval) {
        upper = pivot;
      } else {
        lower = pivot;
      }

      //printf("%s => Lower: %d Upper: %d\n", (pval ? "True" : "False"), lower, upper);
    }

    return (pval ? pivot : pivot + 1);
  }
};

int main(int argc, char **argv)
{
  int N = 10000;

  srand(time(nullptr));
  badVersion = rand() % (N - 1) + 1;
  printf("Bad version: %d\n", badVersion);
  apiCalls = 0;

  Solution sol;
  int v = sol.firstBadVersion(N);

  if (v == badVersion) {
    printf("Found bad version %d out of %d in %d calls\n", v, N, apiCalls);
  } else {
    printf("WRONG: Found %d, correct is %d (out of %d) in %d calls\n", v,
      badVersion, N, apiCalls);
  }

  return 0;
}