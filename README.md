This project is a LOGO language interpreter. The project is divided into several parts: file reading, lexical/grammatical analysis/virtual machine execution.
This project is an unfinished project, only the most basic drawing functions in LOGO language, assignment and loop functions are under development.
The detailed solution for the lexer is as follows:
Here's a detailed breakdown of what my code does:

pub enum Token {...}: This is the definition of the Token enum, which represents the different types of tokens that this lexer can produce. It includes Integer for integer numbers, Name for identifiers, String for string literals, and Eos for the end of the input.

pub struct Lex {...}: This is the definition of the Lex struct, which represents the state of the lexer. It includes input, which is the file being read, and ahead, which is the next token to be returned.

pub fn new(input: File) -> Self {...}: This is the constructor for the Lex struct. It takes a File as input and initializes ahead to Token::Eos.

pub fn next(&mut self) -> Token {...}: This method returns the next token from the input. If ahead is Token::Eos, it calls do_next() to get the next token. Otherwise, it replaces ahead with Token::Eos and returns the old value.

fn do_next(&mut self) -> Token {...}: This method reads the next character from the input and returns the corresponding token. It uses a match statement to handle different characters. For example, if the character is a letter or underscore, it calls read_name(ch) to read an identifier.

fn read_char(&mut self) -> char {...}: This method reads the next character from the input.

fn putback_char(&mut self) {...}: This method puts back the last read character into the input.

fn read_number(&mut self, is_negative: bool) -> Token {...}: This method reads an integer number from the input. If is_negative is true, it negates the number.

fn read_name(&mut self, first: char) -> Token {...}: This method reads an identifier from the input. The first parameter is the first character of the identifier.

fn read_string(&mut self, quote: char) -> Token {...}: This method reads a string literal from the input. The quote parameter is the quote character that started the string.

fn skip_comment(&mut self) {...}: This method skips a comment in the input. It reads characters until it finds a newline or carriage return.
