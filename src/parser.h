#ifndef PARSER_H
#define PARSER_H

#include "lexer.h"
#include "ast.h"

typedef struct
{
  Lexer *lexer;
  Token current;
  Token previous;
  int had_error;
  int panic_mode;
} Parser;

// Pratt parser function pointer types
typedef ASTNode *(*NudFn)(Parser *, Token);            // Null Denotation
typedef ASTNode *(*LedFn)(Parser *, ASTNode *, Token); // Left Denotation

typedef struct
{
  NudFn nud;
  LedFn led;
  int lbp;
} ParseRule;

void parser_init(Parser *parser, Lexer *lexer);

ASTProgram parse(Parser *parser);

#endif
