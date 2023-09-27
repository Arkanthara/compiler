#include <string>
#include <iostream>

using namespace std;

void test_reserved_symbol(string test, int i) {
	if (test[i] == '=' || test[i] == '+' || test[i] == '*' || test[i] == '/') {
		cout << "Reserved symbol: " << test[i] << endl;
	}
}

int test_string(string test, int i) {
	string number = "";
	cout << "Reserved symbol: " << test[i] << endl;
	i++;
	while (test[i] != '\'') {
		number.push_back(test[i]);
		i++;
	}
	cout << "String: " << number << endl;
	cout << "Reserved symbol: " << test[i] << endl;
	return i;
}


int test_number(string test, int i) {
	int first = i;
	string number = "";
	do {
		number.push_back(test[i]);
		i++;
	} while (isdigit(test[i]));
	if (test[i] == '.') {
		do {
			number.push_back(test[i]);
			i++;
		} while (isdigit(test[i]));
		cout << "Real: " << number << endl;
	}
	else {
		if (test[first] == '-') {
			cout << "Real: " << number << endl;
		}
		else {
			cout << "Integer: " << number << endl;
		}
	}
	return i;
}

int main(void) {
	string test = "'clk'43345.234*-234+234=/*32424";
	for (int i = 0; i < test.length(); i++) {
		test_reserved_symbol(test, i);
		if (test[i] == '\'') {
			i = test_string(test, i);
		}
		if (isdigit(test[i])||test[i] == '-') {
			i = test_number(test, i);
		}
	}
}

