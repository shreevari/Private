class Solution {
public:
    int kthGrammar(int N, int K) {
        return helper(N-1, K-1);
    }
    
    int helper(int n, int k) {
        return (n == 0) ? 0 : (helper(n - 1, k / 2) + k % 2) % 2;
    }
};
