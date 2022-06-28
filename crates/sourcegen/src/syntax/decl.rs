pub(crate) fn token_name(token: &str) -> &'static str {
    match token {
        "," => "PUNCT_COMMA",
        ";" => "PUNCT_SEMI",
        "." => "PUNCT_DOT",
        ":" => "PUNCT_COLON",
        "::" => "PUNCT_COLON2",
        "_" => "PUNCT_UNDERSCORE",
        "=>" => "PUNCT_ARROW_FAT",
        "(" => "PUNCT_PAREN_START",
        ")" => "PUNCT_PAREN_END",
        "[" => "PUNCT_BRACKET_START",
        "]" => "PUNCT_BRACKET_END",
        "#{" => "PUNCT_MAP_START",
        "{" => "PUNCT_BRACE_START",
        "}" => "PUNCT_BRACE_END",
        "?" => "PUNCT_QUESTION_MARK",
        "+" => "OP_ADD",
        "-" => "OP_SUB",
        "*" => "OP_MUL",
        "/" => "OP_DIV",
        "%" => "OP_MOD",
        "**" => "OP_POW",
        ">>" => "OP_SHIFT_RIGHT",
        "<<" => "OP_SHIFT_LEFT",
        "&" => "OP_BIT_AND",
        "|" => "OP_BIT_OR",
        "^" => "OP_BIT_XOR",
        "=" => "OP_ASSIGN",
        "+=" => "OP_ADD_ASSIGN",
        "-=" => "OP_SUB_ASSIGN",
        "*=" => "OP_MUL_ASSIGN",
        "/=" => "OP_DIV_ASSIGN",
        "%=" => "OP_MOD_ASSIGN",
        "**=" => "OP_POW_ASSIGN",
        ">>=" => "OP_SHIFT_RIGHT_ASSIGN",
        "<<=" => "OP_SHIFT_LEFT_ASSIGN",
        "&=" => "OP_AND_ASSIGN",
        "|=" => "OP_OR_ASSIGN",
        "^=" => "OP_XOR_ASSIGN",
        "==" => "OP_EQ",
        "!=" => "OP_NOT_EQ",
        ">" => "OP_GT",
        ">=" => "OP_GT_EQ",
        "<" => "OP_LT",
        "<=" => "OP_LT_EQ",
        "&&" => "OP_BOOL_AND",
        "||" => "OP_BOOL_OR",
        "!" => "OP_NOT",
        ".." => "OP_RANGE",
        "..=" => "OP_RANGE_INCLUSIVE",
        "this" => "KW_THIS",
        "let" => "KW_LET",
        "const" => "KW_CONST",
        "global" => "KW_GLOBAL",
        "is_shared" => "KW_IS_SHARED",
        "for" => "KW_FOR",
        "do" => "KW_DO",
        "until" => "KW_UNTIL",
        "while" => "KW_WHILE",
        "in" => "KW_IN",
        "loop" => "KW_LOOP",
        "break" => "KW_BREAK",
        "continue" => "KW_CONTINUE",
        "if" => "KW_IF",
        "else" => "KW_ELSE",
        "switch" => "KW_SWITCH",
        "fn" | "Fn" => "KW_FN",
        "private" => "KW_PRIVATE",
        "return" => "KW_RETURN",
        "throw" => "KW_THROW",
        "try" => "KW_TRY",
        "catch" => "KW_CATCH",
        "import" => "KW_IMPORT",
        "export" => "KW_EXPORT",
        "as" => "KW_AS",
        "call" => "KW_CALL",
        "curry" => "KW_CURRY",
        "type_of" => "KW_TYPE_OF",
        "print" => "KW_PRINT",
        "debug" => "KW_DEBUG",
        "eval" => "KW_EVAL",
        "var" => "KW_VAR",
        "static" => "KW_STATIC",
        "goto" => "KW_GOTO",
        "exit" => "KW_EXIT",
        "match" => "KW_MATCH",
        "case" => "KW_CASE",
        "public" => "KW_PUBLIC",
        "protected" => "KW_PROTECTED",
        "new" => "KW_NEW",
        "use" => "KW_USE",
        "with" => "KW_WITH",
        "module" => "KW_MODULE",
        "package" => "KW_PACKAGE",
        "super" => "KW_SUPER",
        "spawn" => "KW_SPAWN",
        "thread" => "KW_THREAD",
        "go" => "KW_GO",
        "sync" => "KW_SYNC",
        "async" => "KW_ASYNC",
        "await" => "KW_AWAIT",
        "yield" => "KW_YIELD",
        "default" => "KW_DEFAULT",
        "void" => "KW_VOID",
        "null" => "KW_NULL",
        "nil" => "KW_NIL",
        "lit_int" => "LIT_INT",
        "lit_float" => "LIT_FLOAT",
        "lit_bool" => "LIT_BOOL",
        "lit_str" => "LIT_STR",
        "lit_char" => "LIT_CHAR",
        "shebang" => "SHEBANG",
        "ident" => "IDENT",
        "comment_line" => "COMMENT_LINE",
        "comment_line_doc" => "COMMENT_LINE_DOC",
        "comment_block" => "COMMENT_BLOCK",
        "comment_block_doc" => "COMMENT_BLOCK_DOC",
        _ => panic!("unknown token {}", token),
    }
}
