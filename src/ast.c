#include "ast.h"
#include <stdio.h>
#include <stdlib.h>

void print_token(const Token *token)
{
    if (token)
    {
        printf("%.*s", token->length, token->start);
    }
}

void free_node(ASTNode *node)
{
    if (!node)
        return;

    switch (node->type)
    {
    case AST_BINARY:
        free_node(node->binary.left);
        free_node(node->binary.right);
        break;
    case AST_UNARY:
        free_node(node->unary.right);
        break;
    case AST_GROUPING:
        free_node(node->grouping.expression);
        break;
    case AST_LET_STATEMENT:
        free_node(node->let_statement.initializer);
        break;
    case AST_ASSIGNMENT:
        free_node(node->assignment.value);
        break;
    case AST_EXPRESSION_STATEMENT:
        free_node(node->expression_statement.expression);
        break;
    case AST_CALL:
        free_node(node->call.callee);
        for (int i = 0; i < node->call.arg_count; i++)
        {
            free_node(node->call.arguments[i]);
        }
        free(node->call.arguments);
        break;
    case AST_FUNCTION_STATEMENT:
        for (int i = 0; i < node->function_statement.body_count; i++)
        {
            free_node(node->function_statement.body[i]);
        }
        free(node->function_statement.body);
        free(node->function_statement.params);
        break;
    case AST_LAMBDA_EXPRESSION:
        for (int i = 0; i < node->lambda.body_count; i++)
        {
            free_node(node->lambda.body[i]);
        }
        free(node->lambda.body);
        free(node->lambda.params);
        break;
    case AST_VARIABLE:
        break;
    case AST_LITERAL:
        break;
    case AST_MATCH_STATEMENT:
        free_node(node->match_statement.value);
        for (int i = 0; i < node->match_statement.arm_count; i++)
        {
            free_node(node->match_statement.arms[i].pattern);
            free_node(node->match_statement.arms[i].expression);
        }
        free(node->match_statement.arms);
        break;
    case AST_ENUM_STATEMENT:
        free(node->enum_statement.variant_names);
        if (node->enum_statement.field_names)
        {
            for (int i = 0; i < node->enum_statement.variant_count; i++)
            {
                free(node->enum_statement.field_names[i]); // free each field array
            }
            free(node->enum_statement.field_names);
        }
        free(node->enum_statement.field_counts);
        break;

    case AST_ERROR:
        // nothing extra
        break;
    default:
        break;
    }

    free(node);
}

void parser_free_ast(ASTProgram *program)
{
    if (!program || !program->nodes)
        return;

    for (int i = 0; i < program->count; i++)
    {
        free_node(program->nodes[i]);
    }
    free(program->nodes);
    program->nodes = NULL;
    program->count = 0;
    program->capacity = 0;
}

void parser_print_ast(ASTProgram *program)
{
    for (int i = 0; i < program->count; i++)
    {
        parser_print_ast_node(program->nodes[i]);
        printf("\n");
    }
}

void parser_print_ast_node(ASTNode *node)
{
    if (!node)
    {
        printf("NULL");
        return;
    }

    switch (node->type)
    {
    case AST_LITERAL:
        print_token(&node->literal.token);
        break;

    case AST_UNARY:
        print_token(&node->unary.op);
        printf("(");
        parser_print_ast_node(node->unary.right);
        printf(")");
        break;

    case AST_BINARY:
        printf("(");
        parser_print_ast_node(node->binary.left);
        printf(" ");
        print_token(&node->binary.op);
        printf(" ");
        parser_print_ast_node(node->binary.right);
        printf(")");
        break;

    case AST_VARIABLE:
        print_token(&node->variable.name);
        break;

    case AST_GROUPING:
        printf("(");
        parser_print_ast_node(node->grouping.expression);
        printf(")");
        break;

    case AST_ASSIGNMENT:
        print_token(&node->assignment.name);
        printf(" = ");
        parser_print_ast_node(node->assignment.value);
        break;

    case AST_CALL:
        parser_print_ast_node(node->call.callee);
        printf("(");
        for (int i = 0; i < node->call.arg_count; i++)
        {
            parser_print_ast_node(node->call.arguments[i]);
            if (i < node->call.arg_count - 1)
                printf(", ");
        }
        printf(")");
        break;

    case AST_PROPERTY_ACCESS:
        parser_print_ast_node(node->property_access.object);
        printf(".");
        print_token(&node->property_access.property);
        break;

    case AST_ERROR:
        printf("<error>");
        break;

    case AST_LET_STATEMENT:
        printf("let ");
        print_token(&node->let_statement.name);
        printf(" = ");
        parser_print_ast_node(node->let_statement.initializer);
        break;

    case AST_LET_BANG_STATEMENT:
        printf("let! ");
        print_token(&node->let_bang_statement.name);
        printf(" = ");
        parser_print_ast_node(node->let_bang_statement.initializer);
        break;

    case AST_EXPRESSION_STATEMENT:
        parser_print_ast_node(node->expression_statement.expression);
        break;

    case AST_FUNCTION_STATEMENT:
        printf("func ");
        print_token(&node->function_statement.name);
        printf("(");
        for (int i = 0; i < node->function_statement.param_count; i++)
        {
            print_token(&node->function_statement.params[i]);
            if (i < node->function_statement.param_count - 1)
                printf(", ");
        }
        printf(") { ");
        for (int i = 0; i < node->function_statement.body_count; i++)
        {
            parser_print_ast_node(node->function_statement.body[i]);
            if (i < node->function_statement.body_count - 1)
                printf("; ");
        }
        printf(" }");
        break;

    case AST_LAMBDA_EXPRESSION:
        printf("fn(");
        for (int i = 0; i < node->lambda.param_count; i++)
        {
            print_token(&node->lambda.params[i]);
            if (i < node->lambda.param_count - 1)
                printf(", ");
        }
        printf(") -> { ");
        for (int i = 0; i < node->lambda.body_count; i++)
        {
            parser_print_ast_node(node->lambda.body[i]);
            if (i < node->lambda.body_count - 1)
                printf("; ");
        }
        printf(" }");
        break;

    case AST_MATCH_STATEMENT:
        printf("match ");
        parser_print_ast_node(node->match_statement.value);
        printf(" {\n");
        for (int i = 0; i < node->match_statement.arm_count; i++)
        {
            printf("  ");
            parser_print_ast_node(node->match_statement.arms[i].pattern);
            printf(" -> ");
            parser_print_ast_node(node->match_statement.arms[i].expression);
            printf(",\n");
        }
        printf("}");
        break;

    case AST_PIPELINE:
        printf("(");
        parser_print_ast_node(node->pipeline.left);
        printf(" |> ");
        parser_print_ast_node(node->pipeline.right);
        printf(")");
        break;

    case AST_IMPORT_STATEMENT:
        printf("import ");
        print_token(&node->import_statement.path);
        break;

    case AST_LIST_LITERAL:
        printf("[");
        for (int i = 0; i < node->list_literal.count; i++)
        {
            parser_print_ast_node(node->list_literal.elements[i]);
            if (i < node->list_literal.count - 1)
                printf(", ");
        }
        printf("]");
        break;

    case AST_STRUCT_LITERAL:
        printf("{ ");
        for (int i = 0; i < node->struct_literal.count; i++)
        {
            print_token(&node->struct_literal.keys[i]);
            printf(" = ");
            parser_print_ast_node(node->struct_literal.values[i]);
            if (i < node->struct_literal.count - 1)
                printf(", ");
        }
        printf(" }");
        break;

    case AST_STRUCT_UPDATE:
        parser_print_ast_node(node->struct_update.base);
        printf(" <- { ");
        for (int i = 0; i < node->struct_update.count; i++)
        {
            print_token(&node->struct_update.keys[i]);
            printf(" = ");
            parser_print_ast_node(node->struct_update.values[i]);
            if (i < node->struct_update.count - 1)
                printf(", ");
        }
        printf(" }");
        break;
    case AST_ENUM_STATEMENT:
        printf("enum ");
        print_token(&node->enum_statement.name);
        printf(" {\n");
        for (int i = 0; i < node->enum_statement.variant_count; i++)
        {
            printf("  ");
            print_token(&node->enum_statement.variant_names[i]);
            if (node->enum_statement.field_counts[i] > 0)
            {
                printf(" { ");
                for (int j = 0; j < node->enum_statement.field_counts[i]; j++)
                {
                    print_token(&node->enum_statement.field_names[i][j]);
                    if (j < node->enum_statement.field_counts[i] - 1)
                        printf(", ");
                }
                printf(" }");
            }
            if (i < node->enum_statement.variant_count - 1)
                printf(",");
            printf("\n");
        }
        printf("}");
        break;

    case AST_BOOL_LITERAL:
        printf(node->bool_literal.value ? "true" : "false");
        break;
    default:
        printf("<?>");
        break;
    }
}
