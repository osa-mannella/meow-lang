#ifndef AST_H
#define AST_H

#include "lexer.h"
#include <stdbool.h>

typedef enum
{
  AST_LITERAL,
  AST_BINARY,
  AST_UNARY,
  AST_VARIABLE,
  AST_GROUPING,
  AST_ASSIGNMENT,
  AST_CALL,
  AST_ERROR,
  AST_LET_STATEMENT,
  AST_EXPRESSION_STATEMENT,
  AST_FUNCTION_STATEMENT,
  AST_LAMBDA_EXPRESSION,
  AST_MATCH_STATEMENT,
  AST_PROPERTY_ACCESS,
  AST_LET_BANG_STATEMENT,
  AST_PIPELINE,
  AST_IMPORT_STATEMENT,
  AST_LIST_LITERAL,
  AST_STRUCT_LITERAL,
  AST_STRUCT_UPDATE,
  AST_BOOL_LITERAL,
  AST_ENUM_STATEMENT,
  AST_ENUM_CONSTRUCTOR,
  AST_DESTRUCTURE_PATTERN,

} ASTNodeType;

typedef struct ASTNode ASTNode;

typedef struct
{
  ASTNode *pattern;
  ASTNode *expression;
} MatchArm;

struct ASTNode
{
  ASTNodeType type;
  union
  {
    struct
    {
      Token token;
    } literal;
    struct
    {
      Token op;
      ASTNode *right;
    } unary;
    struct
    {
      ASTNode *left;
      Token op;
      ASTNode *right;
    } binary;
    struct
    {
      Token name;
    } variable;
    struct
    {
      ASTNode *expression;
    } grouping;
    struct
    {
      Token name;
      ASTNode *value;
    } assignment;
    struct
    {
      ASTNode *callee;
      ASTNode **arguments;
      int arg_count;
    } call;
    struct
    {
      Token name;
      ASTNode *initializer;
    } let_statement;
    struct
    {
      ASTNode *expression;
    } expression_statement;
    struct
    {
      Token name;
      Token *params;
      int param_count;
      ASTNode **body;
      int body_count;
    } function_statement;
    struct
    {
      Token *params;
      int param_count;
      ASTNode **body;
      int body_count;
    } lambda;

    struct
    {
      ASTNode *value;
      MatchArm *arms;
      int arm_count;
    } match_statement;

    struct
    {
      ASTNode *object;
      Token property;
    } property_access;
    struct
    {
      Token name;
      ASTNode *initializer;
    } let_bang_statement;
    struct
    {
      ASTNode *left;
      ASTNode *right;
    } pipeline;
    struct
    {
      Token path;
    } import_statement;
    struct
    {
      ASTNode **elements;
      int count;
    } list_literal;
    struct
    {
      Token *keys;
      ASTNode **values;
      int count;
    } struct_literal;
    struct
    {
      ASTNode *base;
      Token *keys;
      ASTNode **values;
      int count;
    } struct_update;
    struct
    {
      bool value;
    } bool_literal;
    struct
    {
      Token name;           // enum name
      Token *variant_names; // variant names
      Token **field_names;  // each variant's fields (array of arrays)
      int *field_counts;    // number of fields per variant
      int variant_count;
    } enum_statement;
    struct
    {
      Token enum_name;
      Token variant_name;
      Token *field_names; // ["radius"]
      ASTNode **values;   // [Literal(4)]
      int field_count;
    } enum_constructor;
    struct
    {
      Token *bindings; // array of field bindings
      int binding_count;
    } destructure_pattern;
  };
};

typedef struct
{
  ASTNode **nodes;
  int count;
  int capacity;
} ASTProgram;

void free_node(ASTNode *node);
void parser_free_ast(ASTProgram *program);
void parser_print_ast(ASTProgram *program);
void parser_print_ast_node(ASTNode *node);

#endif
