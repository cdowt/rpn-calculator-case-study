#include "rpn_calculator.h"

#define MAX_TERMS 32

enum error {
	NO_ERROR,
};

enum term_type {
	VALUE,
	OPERATOR,
};

enum operator{
	PLUS,
	MINUS,
	MULTIPLY,
	DIVIDE,
	REMAINDER,
};

struct term {
	enum term_type type;
	union {
		int value;
		enum operator operator;
	};
};

struct expression {
	struct term terms[MAX_TERMS];
	unsigned term_count;
};

void run_repl(void)
{
}
