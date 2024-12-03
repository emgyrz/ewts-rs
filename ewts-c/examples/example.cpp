/* g++ ../../target/release/libewts.dylib example.cpp */

#include "../ewts-cpp.h"
#include <string>
#include <fstream>
#include <iostream>

using namespace std;


string read_sample_ewts_text() {
  string filePath = "sample.txt";

  ifstream file(filePath);

  if (!file.is_open()) {
    cerr << "Failed to open file: " << filePath << endl;

    return "";
  }

  string result;
  string line;
  while (getline(file, line)) {
    result += line + "\n";
  }

  file.close();

  return result;
}




int main()
{
  string src = read_sample_ewts_text();
  const char * converted = ewts_to_unicode(src.c_str());

  printf("Input string is:\n%s", src.c_str());
  printf("Converted string is:\n%s", converted);

  free_ewts_string(converted);
}

