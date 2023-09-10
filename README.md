# cpp_gen
C++ code generator

## Grammar for C++14 for antlr4

https://github.com/antlr/grammars-v4/blob/master/cpp/CPP14Parser.g4

To make the grammar work in rust, modifications to the generated cpp14parser.rs file and
the CPP14parser.g4 file is necessary. First change CPP14Parser.g4 to reflect rust syntax.
This has to be done once only. After that, generate the rust classes and add a missing
function. This has to be done everytime the grammar is rebuild.

Change the occurences of 

```
this.IsPureSpecifierAllowed() 
```

inside CPP14Parser.g4 to 

```
IsPureSpecifierAllowed()
```

Then rebuild the grammar. A explanation on how to build the grammar is contained in the next section.

Insert a global function 'fn IsPureSpecifierAllowed() -> bool' somewhere inside cpp14parser.rs (e.g. on line 500):

```
fn IsPureSpecifierAllowed() -> bool
{
    panic!("Implement this!");

	// try
	// {
	// 	auto x = this->getRuleContext(); // memberDeclarator
	// 	auto c = x->children[0]->children[0];
	// 	auto c2 = c->children[0];
	// 	auto p = c2->children[1];
	// if (p == nullptr) return false;
	// return typeid(*p) == typeid(CPP14Parser::ParametersAndQualifiersContext);
	// }
	// catch (...)
	// {
	// }
	// return false;

	//return false;
}
```

In cpp14parser.rs replace all occurences of 1isize with 1i64. Otherwise there will be shift with overflow errors.

Then rebuild the application

```
cargo build
```

## Generate using Java JVM from the command line

First generate the lexer.

```
cd resources/antlr4
java -jar antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust -o ../../src/parser ../../src/parser/CPP14Lexer.g4 -visitor
cd ../..
```

Then generate the parser.

```
cd resources/antlr4
java -jar antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust -o ../../src/parser ../../src/parser/CPP14Parser.g4 -visitor
cd ../..
```

Or do everything in one go:

```
cd resources/antlr4
java -jar antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust -o ../../src/parser ../../src/parser/CPP14Lexer.g4 -visitor
java -jar antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust -o ../../src/parser ../../src/parser/CPP14Parser.g4 -visitor
cd ../..
```

