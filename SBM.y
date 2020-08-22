%{
#include <stdio.h>
#include <string.h>
#include <math.h>

int yylex();
void yyerror(const char *message);
%}
%union {
int ival;
}
%token ADD
%token SUB
%token MUL
%token MOD
%token LOAD
%token INC
%token DEC
%token DELETE
%token SWITCH
%token COPY
%token END_OF_FILE;
%token <ival> INUMBER
%type <ival> exp
%type <ival> cpy 

%%
input : line  {return 0; }
      ;
line : exp  /*END_OF_FILE*/  { printf ("%d\n", $1); }
     ; 
exp : LOAD INUMBER   { $$ = $2;         }
   | exp exp ADD     { $$ = $2 + $1;    }
   | exp exp SUB     { $$ = $2 - $1;    }
   | exp exp MUL     { $$ = $2 * $1;    }
   | exp exp MOD     { $$ = $2 % $1;    }
   | exp INC         { $$ = $1 + 1 ;    }
   | exp DEC         { $$ = $1 - 1 ;    }
   | exp exp DELETE  { $$ = $1;   }
   | exp COPY        { $2 = $1;   }             // Have no idea how to impliment!
   | exp exp SWITCH  { int tmp=$1; $1=$2; $2=tmp;   }   
   ; 
%%
void yyerror (const char *message)
{
        printf ("Invalid format\n");
}
int main(int argc, char *argv[]) {
        yyparse();
        return(0);
}
