//
// Created by Matt on 4/25/2019.
//

#ifndef MANGO_CL_EXPR_H
#define MANGO_CL_EXPR_H


#include <string>
#include <list>
#include <map>
#include <iostream>

class expr
{
public:
    float number;

    virtual void print() = 0;
    virtual float evaluate() = 0;
};

class operator_node : public expr
{
public:
    expr *left;
    expr *right;
};

class number_node : public expr
{
public:
    number_node(float);
    void print();
    float evaluate();
};

class id_node : public expr
{
protected:
    std::string id;

public:
    id_node(std::string);
    void print();
    float evlatuate();
};

class plus_node : public operator_node
{
public:
    plus_node(expr *, expr *);
    void print();
    float evaluate();
};

class minus_node : public operator_node
{
public:
    minus_node(expr *, expr *);
    void print();
    float evaluate();
};

class mult_node : public operator_node
{
public:
    mult_node(expr *, expr *);
    void print();
    float evaluate();
};

class div_node : public operator_node
{
public:
    div_node(expr *, expr *);
    void print();
    float evaluate();
};

class statement
{
public:
    virtual void print() {}
    virtual void evaluate() = 0;
};

class assignment_statement : public statement
{
protected:
    std::string id;
    expr *expr;

public:
    assignment_statement(std::string name, class expr *expression);
    void print();
    void evaluate();
};

class mangopgrm
{
protected:
    std::list<statement *> *stmts;

public:
    mangopgrm(std::list<statement *> *stmtlist);
    void evaluate();
};

extern std::map<std::string, float> idTable;
extern mangopgrm *root;


#endif //MANGO_CL_EXPR_H
