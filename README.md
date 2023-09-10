# cpp_gen
C++ code generator

## Generate using Java JVM from the command line

```
cd resources\antlr4
java -jar antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust -o ../../src/parser ../../src/parser/CPP14Lexer.g4
```
