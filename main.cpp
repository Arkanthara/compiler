#include <string>
#include "tokenize.h"

using namespace std;

int main(void) {
	string test = "'clk'43345.234*-234+234=/*32424";
	for (int i = 0; i < test.length(); i++) {
		test_reserved_symbol(test, &i);
		if (test[i] == '\'') {
			test_string(test, &i);
		}
		if (isdigit(test[i])||test[i] == '-') {
			test_number(test, &i);
		}
	}
}
