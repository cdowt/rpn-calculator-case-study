#include "rpn_calculator.h"

#include "io.h"

#define PROMPT "> "
#define MAX_TERMS 32
#define MAX_DIGITS 20

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

static const char *crlf = "\r\n";

static enum error read_expression(struct expression *expression_out);
static enum error evaluate(
	const struct expression *expression, int *result_out);

static void print_error(enum error e);
static void print_result(int result);

void run_repl(void)
{
	static struct expression expression;
	enum error e;
	int result;

	while (1) {
		if ((e = read_expression(&expression)) != NO_ERROR
			|| (e = evaluate(&expression, &result)) != NO_ERROR)
			print_error(e);
		else
			print_result(result);
	}
}

static enum error read_expression(struct expression *expression_out)
{
	return NO_ERROR;
}

static enum error evaluate(const struct expression *expression, int *result_out)
{
	*result_out = 42;
	return NO_ERROR;
}

static void print_error(enum error e) { }

static void print_result(int result)
{
	if (result < 0) {
		print_char('-');
		result *= -1;
	}

	char digits[MAX_DIGITS];
	unsigned n = 0;
	do {
		digits[n++] = '0' + result % 10;
		result /= 10;
	} while (result != 0);

	while (--n < MAX_DIGITS)
		print_char(digits[n]);

	print_str(crlf);
}
