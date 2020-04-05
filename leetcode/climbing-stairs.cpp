#include <map>
class Solution {
public:
    map<int, int> cache;
    int climbStairs(int n) {
        if (n < 4)
            return n;
        else {
            int previous = cache[n-1];
            int before_that = cache[n-2];
            if (previous == 0){
                previous = climbStairs(n - 1);
                cache[n-1] = previous;
            }
            if (before_that == 0){
                before_that = climbStairs(n - 2);
                cache[n-2] = before_that;
            }
            return previous + before_that;
        }
    }
};
