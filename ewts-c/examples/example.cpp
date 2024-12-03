/*clang -std=c++11 -stdlib=libc++ -Weverything ../../target/release/libewts.dylib example.cpp*/

#include "../ewts-cpp.h"

int main()
{
  char str[] = "ka kha /";

  printf("Converter string is: %s \n", ewts_to_unicode(str));
}
