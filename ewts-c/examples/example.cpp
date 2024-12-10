/* g++ ../../target/release/libewts.dylib example.cpp -O2 && ./a.out */

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
  uintptr_t converter_ptr = create_ewts_converter();

  const char * converted_str = ewts_to_unicode(converter_ptr, src.c_str());

  printf("Input string is:\n%s\n", src.c_str());
  printf("Converted string is:\n%s", converted_str);

  free_ewts_string(converted_str);
  free_ewts_converter(converter_ptr);
}

