#include<iostream>

using namespace std;

class Solution {
public:
  int reverse(int x) {

    bool is_negative = false;
    if (x < 0) {
      is_negative = true;
      x = -x;
    }

    int reverse_num = 0;
    while (x > 0) {
      int digit = x % 10;
      reverse_num = reverse_num * 10 + digit;
      x /= 10;
    }
    
    if (is_negative)
      reverse_num = -reverse_num;

    return reverse_num;
  }
};

int main(int argc, char* argv[]) {
  
  if(argc != 2)
    return 1;
  else {
    Solution s;
    cout << s.reverse(atoi(argv[1]));
  }
  return 0;
}
