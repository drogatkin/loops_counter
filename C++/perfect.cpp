#include <iostream>
using namespace std;

int main() {
    int num, sum;
    sum = 0;
    num = 20'000;
    
    for (int j = num; j >= 1; j--) {
        for ( int i = num - 1; i >= 1; i--) {
            if (num % i == 0) {
                sum += i;
            }
        }
        if (num == sum) {
            cout << "num: " << num << "/ sum:" << sum << endl;
        }
        sum = 0;
        num --;
    }
}