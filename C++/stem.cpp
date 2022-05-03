// C++ program to find the stem of given list of
// words
#include <iostream>
#include <vector>
#include <algorithm>
#include <fstream>
#include <nlohmann/json.hpp>

using namespace std;
using json = nlohmann::json;
 
// function to find the stem (longest common
// substring) from the string array
vector<string> findstem(vector<string> arr)
{
    // Determine size of the array
    sort(arr.begin(), arr.end(), []
    (const std::string& first, const std::string& second){
        return first.size() < second.size();
    });
    
    int n = arr.size();
    // Take first word from array as reference

    string s = arr[0];

    int len = s.length();
 
    vector<string> res;

    for (int i = len; i > 0; i--) {
        
        for (int j = 0; j < len-i+1; j++) {

            // generating all possible substrings
            // of our reference string arr[0] i.e s
            string stem = s.substr(j, i);

            int k;

            for (k = 1; k < n; k++) {
                // Check if the generated stem is
                // common to all words
                if (arr[k].find(stem) == std::string::npos)
                    // not found
                    break;

            }
           // cout << "looking for " << stem << " and found in " << k << " end " << n << endl;
            // If current substring is present in all strings
            if (k == n)
                 res.push_back(stem);
        }
        if (res.size() > 0)
          break;
    }
 
    return res;
}
 
// Driver code
int main(int pl, char *params[])
{

   // vector<string> arr{ "grace", "graceful", "disgraceful",
   //                     "gracefully" };
    json stems;
    char fileName[] = "strings.json";
    char *name = fileName;
    if (pl > 1)
       name = params[1];
    ifstream stem_file(name, ifstream::binary);
    stem_file >> stems;

    // Function call

    vector<string> stem1 = findstem(stems);
    cout << "[  ";
    for (string i: stem1)
        cout << i << "  ";
    cout << "]" << endl;
}
 
