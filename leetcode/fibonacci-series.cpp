#include <map>
class Solution {
public:
    map<int, int> cache;
    int fib(int N) {
        if (N < 2) {
            return N;
        }
        int cached = cache[N];
        if (cached == 0) {
            return fib(N - 1) + fib(N - 2);
        } else {
            return cached;
        }
    }
};
