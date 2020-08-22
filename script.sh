bison -d -o y.tab.c SBM.y
flex -o lex.yy.c SBM.l
gcc -c -g -I.. y.tab.c
gcc -c -g -I.. lex.yy.c
gcc lex.yy.o y.tab.o -ll
