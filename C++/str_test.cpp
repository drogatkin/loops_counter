#include <sstream>
#include <string>
#include <iostream>

std::string ran_str( size_t length )
{
    auto randchar = []() -> char
    {
        const char charset[] =
        "0123456789"
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        "abcdefghijklmnopqrstuvwxyz";
        const size_t max_index = (sizeof(charset) - 1);
        return charset[ rand() % max_index ];
    };
    std::string str(length,0);
    std::generate_n( str.begin(), length, randchar );
    return str;
}

int main() {
    std::stringstream ss;
   std::string ran_vals[5] = {ran_str(6), ran_str(25), ran_str(14), ran_str(235) , ran_str(155)};
   auto res = 0;
   for (int i=1; i < 100'000; ++i) {
        if (i == 1)
           ss << ran_vals[0];
        else if(i == 2) 
           ss << ran_vals[0] << ran_vals[1];
        else if(i == 3) 
           ss << ran_vals[1] << ran_vals[2]  << ran_vals[0];
        else if(i == 4) 
           ss << ran_vals[3] << ran_vals[1]  << ran_vals[2] << ran_vals[0];
        else if(i == 5) 
           ss << ran_vals[3] << ran_vals[1]  << ran_vals[2] << ran_vals[0] << ran_vals[4];
        else {
            ss << ran_vals[4];
            for (int j= 1; j < i; ++j )
                 ss << ran_vals[j % sizeof(ran_vals)];
        }
        std::string str = ss.str();
        std::cout << "String[" << i << "]: " << str << '\n';
        std::size_t found = str.find("A", 0);
        if (found!=std::string::npos)
            res +=  found;
        found = str.rfind("z", 0);
        if (found!=std::string::npos)
            res +=  found;   
   }
   std::cout << "Result: " << res << '\n';
}


