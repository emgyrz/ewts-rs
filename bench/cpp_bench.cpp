
#include "../ewts-c/ewts-cpp.h"
#include <chrono>
#include <cmath>
#include <filesystem>
#include <string>
#include <fstream>
#include <iostream>

using namespace std;

const int ITERATION_COUNT = 33;
const char * file_path = "sample_ewts_text.txt";


string read_sample_ewts_text() {
  ifstream file(file_path);

  string result;
  string line;
  while (getline(file, line)) {
    result += line + "\n";
  }

  file.close();
  return result;
}


int utf8_strlen(const string& str)
{
    int c,i,ix,q;
    for (q=0, i=0, ix=str.length(); i < ix; i++, q++)
    {
        c = (unsigned char) str[i];
        if      (c>=0   && c<=127) i+=0;
        else if ((c & 0xE0) == 0xC0) i+=1;
        else if ((c & 0xF0) == 0xE0) i+=2;
        else if ((c & 0xF8) == 0xF0) i+=3;
        else return 0;
    }
    return q;
}


int main() {
  string src = read_sample_ewts_text();
  const char * s = src.c_str();
  uintptr_t converter_ptr = create_ewts_converter();
  int len = 0;

  auto start = chrono::high_resolution_clock::now();

  for (int i = 0; i < ITERATION_COUNT; i++) {
    const char * converted = ewts_to_unicode(converter_ptr, s);
    len += utf8_strlen(converted);
    free_ewts_string(converted);
  }

  auto elapsed_ms = duration_cast<chrono::milliseconds>(chrono::high_resolution_clock::now() - start);

  free_ewts_converter(converter_ptr);

  double_t speed = (filesystem::file_size(file_path) * ITERATION_COUNT / 1024.0) / (elapsed_ms.count() / 1000.0);
  cout << "ewts-rs (c++ bindings): speed - " << speed << " Kb/s; ";
  cout << "launches - " << ITERATION_COUNT << ";";
  cout << "time - " << elapsed_ms.count() << " ms\n";
}


