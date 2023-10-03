#include <string>
#include <iostream>

using namespace std;

char test_reserved_symbol(string test, int * i) {
        if (test[*i] == '=' || test[*i] == '+' || test[*i] == '*' || test[*i] == '/' || test[*i] == '(' || test[*i] == ')') {
		cout << "Reserved symbol: " << test[*i] << endl;
	}
	return test[*i];
}

string test_string(string test, int *i) {
	string text = "";
	cout << "Reserved symbol: " << test[*i] << endl;
	*i++;
	while (test[*i] != '\'') {
		text.push_back(test[*i]);
		*i++;
	}
	cout << "String: " << text << endl;
	cout << "Reserved symbol: " << test[*i] << endl;
	return text;
}


double test_number(string test, int *i) {
	int first = *i;
	string number = "";
	do {
		number.push_back(test[*i]);
		*i++;
	} while (isdigit(test[*i]));
	if (test[*i] == '.') {
		do {
			number.push_back(test[*i]);
			*i++;
		} while (isdigit(test[*i]));
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
	return stod(number);
}
