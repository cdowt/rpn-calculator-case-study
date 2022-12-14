#include "rpn_calculator.h"

#include "io.h"

#define PROMPT "> "
#define MAX_TERMS 32
#define MAX_DIGITS 20
#define MAX_TOKEN_LENGTH 16
#define STACK_SIZE 32

enum error {
	NO_ERROR,
	INVALID_TERM,
	TOO_MANY_TERMS,
	TOKEN_TOO_LONG,
	STACK_OVERFLOW,
	STACK_UNDERFLOW,
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

struct stack {
	int values[STACK_SIZE];
	unsigned top_index;
};

static const char *crlf = "\r\n";
static const char *error_messages[] = {
	[INVALID_TERM] = "a term in the expression was invalid",
	[TOO_MANY_TERMS] = "too many terms in the expression",
	[TOKEN_TOO_LONG] = "a term in the expression was too long",
	[STACK_OVERFLOW] = "the stack overflowed",
	[STACK_UNDERFLOW] = "the stack underflowed",
};

static enum error read_expression(struct expression *expression_out);
static enum error evaluate(
	const struct expression *expression, int *result_out);

static void print_error(enum error e);
static void print_result(int result);
static enum error read_token(char token[MAX_TOKEN_LENGTH],
	unsigned *token_length_out, short *end_of_line_out);
static short try_read_value(
	char token[MAX_TOKEN_LENGTH], unsigned token_length, struct term *term_out);
static short try_read_operator(
	char token[MAX_TOKEN_LENGTH], unsigned token_length, struct term *term_out);
static enum error push(struct stack *stack, int value);
static enum error pop(struct stack *stack, int *value_out);
static int apply(enum operator operator, int first, int second);

void run_repl(void)
{
	static struct expression expression;
	enum error e;
	int result;

	while (1) {
		print_str(PROMPT);
		if ((e = read_expression(&expression)) != NO_ERROR
			|| (e = evaluate(&expression, &result)) != NO_ERROR)
			print_error(e);
		else
			print_result(result);
	}
}

static enum error read_expression(struct expression *expression_out)
{
	expression_out->term_count = 0;

	static char token[MAX_TOKEN_LENGTH];
	unsigned token_length;
	short end_of_line;
	enum error e;

	while (1) {
		if ((e = read_token(token, &token_length, &end_of_line)) != NO_ERROR)
			return e;

		if (token_length == 0)
			continue;

		struct term *current_term
			= &expression_out->terms[expression_out->term_count];
		if (try_read_operator(token, token_length, current_term)
			|| try_read_value(token, token_length, current_term))
			++expression_out->term_count;
		else
			return INVALID_TERM;

		if (expression_out->term_count == MAX_TERMS)
			return TOO_MANY_TERMS;

		if (end_of_line)
			break;
	}

	return NO_ERROR;
}

static enum error evaluate(const struct expression *expression, int *result_out)
{
	int first, second;
	struct stack stack = { 0 };
	enum error e;
	for (unsigned i = 0; i < expression->term_count; ++i) {
		switch (expression->terms[i].type) {
		case VALUE:
			if ((e = push(&stack, expression->terms[i].value)) != NO_ERROR)
				return e;
			break;

		case OPERATOR:
			if ((e = pop(&stack, &second)) != NO_ERROR)
				return e;
			if ((e = pop(&stack, &first)) != NO_ERROR)
				return e;
			push(&stack, apply(expression->terms[i].operator, first, second));
			break;
		}
	}

	return pop(&stack, result_out);
}

static void print_error(enum error e)
{
	print_str(crlf);
	print_str("Error: ");
	print_str(error_messages[e]);
	print_str(crlf);
}

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

static enum error read_token(char token[MAX_TOKEN_LENGTH],
	unsigned *token_length_out, short *end_of_line_out)
{
	char c;
	unsigned length = 0;
	while (1) {
		switch (c = read_char()) {
		case ' ':
		case '\t':
			*token_length_out = length;
			*end_of_line_out = 0;
			return NO_ERROR;

		case '\r':
			print_char('\n');
			*token_length_out = length;
			*end_of_line_out = 1;
			return NO_ERROR;

		default:
			if (length == MAX_TOKEN_LENGTH)
				return TOKEN_TOO_LONG;
			else
				token[length++] = c;
		}
	}
	return NO_ERROR;
}

static short try_read_value(
	char token[MAX_TOKEN_LENGTH], unsigned token_length, struct term *term_out)
{
	int sign;
	unsigned digits_start;
	if (token[0] == '-') {
		sign = -1;
		digits_start = 1;
	} else {
		sign = 1;
		digits_start = 0;
	}

	int magnitude = 0;
	for (unsigned i = digits_start; i < token_length; ++i) {
		if (token[i] < '0' || token[i] > '9')
			return 0;
		magnitude *= 10;
		magnitude += token[i] - '0';
	}

	term_out->type = VALUE;
	term_out->value = sign * magnitude;
	return 1;
}

static short try_read_operator(
	char token[MAX_TOKEN_LENGTH], unsigned token_length, struct term *term_out)
{
	if (token_length != 1)
		return 0;

	enum operator operator;
	switch (token[0]) {
	case '+':
		operator= PLUS;
		break;
	case '-':
		operator= MINUS;
		break;
	case '*':
		operator= MULTIPLY;
		break;
	case '/':
		operator= DIVIDE;
		break;
	case '%':
		operator= REMAINDER;
		break;

	default:
		return 0;
	}

	term_out->type = OPERATOR;
	term_out->operator= operator;
	return 1;
}

static enum error push(struct stack *stack, int value)
{
	if (stack->top_index == STACK_SIZE)
		return STACK_OVERFLOW;
	stack->values[stack->top_index++] = value;
	return NO_ERROR;
}

static enum error pop(struct stack *stack, int *value_out)
{
	if (stack->top_index == 0)
		return STACK_UNDERFLOW;
	*value_out = stack->values[--stack->top_index];
	return NO_ERROR;
}

static int apply(enum operator operator, int first, int second)
{
	switch (operator) {
	case PLUS:
		return first + second;
	case MINUS:
		return first - second;
	case MULTIPLY:
		return first * second;
	case DIVIDE:
		return first / second;
	case REMAINDER:
		return first % second;

	default:
		return 0;
	}
}
