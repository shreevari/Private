#include <map>
class Solution {
public:
    map<int, double> cache;    
    double myPow(double x, int n) {
        if (x == 0 || x == 1)
            return x;
        if (x == -1)
            return (n % 2 == 0) ? 1 : -1; 
        if (n > 0)
            return multiplier(x, n);
        else
            return divider(x, n);
    }
    double multiplier(double x, int n) {
        if (n == 0)
            return 1;
        if (cache[n / 2] == 0) {
            cache[n / 2] = multiplier(x, n / 2);
        }
        if (n % 2 == 1) 
            return cache[n / 2] * cache[n / 2] * x;
        else
            return cache[n / 2] * cache[n / 2];
    }
    double divider(double x, int n) {
        if (n == 0)
            return 1;
        if (cache[n / 2] == 0) {
            cache[n / 2] = divider(x, n / 2);
        }
        if (n & 1 == 1) 
            return cache[n / 2] * cache[n / 2] * (1 / x);
        else
            return cache[n / 2] * cache[n / 2];
    }    
};
