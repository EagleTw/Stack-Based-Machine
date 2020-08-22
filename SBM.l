%option noyywrap
%{
#include "y.tab.h"
%}

%%
\n        {}
[ \t]+	  { /* do nothing */ }
"add"     {return ADD;}
"sub"	  {return SUB;}
"mul"     {return MUL;}
"mod"     {return MOD;}
"inc"	  {return INC;}
"dec"	  {return DEC;}
"load"    {return LOAD;}
"delete"  {return DELETE;}
"switch"  {return SWITCH;}
"copy"    {return COPY;}
[0-9]+	  { yylval.ival= atoi(yytext);return(INUMBER); }
.	      { return(yytext[0]); }
<<EOF>>   {return END_OF_FILE; }
%%
