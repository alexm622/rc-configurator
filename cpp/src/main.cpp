//main.cpp

#include <string>
#include <iostream>
#include <filesystem>

#include <vector>
#include <string>

int main(){
  std::string path = "/etc/rc.d";
  std::vector<std::string> entries;
  for (const auto &entry : std::filesystem::directory_iterator(path)) {
    //std::cout << entry.path() << std::endl;
    entries.push_back(entry.path());
  }

  for (long unsigned int i = 0; i < entries.size(); i++) {
    printf("%lu: %s\n",i,entries.at(i).c_str());
  }
}
