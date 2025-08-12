// pub enum Token {
//     // EOF
//     Eof,
//     // WhiteSpace, LineTerminator, Comment
//     Skip,
//     // Reserved Words
//     Abstract,
//     Async,
//     Await,
//     Break,
//     Case,
//     Catch,
//     Class,
//     Const,
//     Continue,
//     Debugger,
//     Declare,
//     Default,
//     Delete,
//     Do,
//     Else,
//     Enum,
//     Export,
//     Extends,
//     False,
//     Finally,
//     For,
//     Function,
//     If,
//     Implements,
//     Import,
//     In,
//     Instanceof,
//     Interface,
//     Is,
//     Let,
//     Module,
//     Namespace,
//     New,
//     Null,
//     Override,
//     Package,
//     Private,
//     Protected,
//     Public,
//     Readonly,
//     Require,
//     Return,
//     Static,
//     Super,
//     Switch,
//     This,
//     Throw,
//     True,
//     Try,
//     Type,
//     Typeof,
//     Var,
//     Void,
//     While,
//     With,
//     Yield,
//     // Token
//     LBrace,
//     RBrace,
//     LParen,
//     RParen,
//     LBracket,
//     RBracket,
//     Dot,
//     DotDotDot,
//     Semi,
//     Comma,
//     LessThan,
//     LessThanEquals,
//     GreaterThan,
//     GreaterThanEquals,
//     Equals,
//     EqualsEquals,
//     EqualsEqualsEquals,
//     BangEquals,
//     BangEqualEqual,
//     Plus,
//     PlusPlus,
//     Minus,
//     MinusMinus,
//     Star,
//     StarStar,
//     Percent,
//     LessThanLessThan,
//     GreaterThanGreaterThan,
//     GreaterThanGreaterThanGreaterThan,
//     Ampersand,
//     AmpersandAmpersand,
//     Bar,
//     BarBar,
//     Caret,
//     Bang,
//     Tilde,
//     Question,
//     QuestionQuestion,
//     Colon,
//     PlusAssign,
//     MinusAssign,
//     StarAssign,
//     PercentAssign,
//     StarStarAssign,
//     LAngleLAngleAssign,
//     RAngleRAngleAssign,
//     RAngleRAngleRAngleAssign,
//     AmpersandAssign,
//     AmpersandAmpersandAssign,
//     BarAssign,
//     BarBarAssign,
//     CaretAssign,
//     QuestionQuestionAssign,
//     Arrow,
//     Slash,
//     SlashEqual,
//     At,
//     // Literal
//     Numeric,
//     String,
//     Regex,
//     TemplateHead,
//     TemplateMiddle,
//     TemplateTail,
//     NoSubstitutionTemplate,
//     // Identifier
//     Identifier,
//     // Type Operator
//     Keyof,
//     Unique,
// }

// =================================================================
// This snippet is from the 'microsoft/typescript-go' project.
//
// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Source: https://github.com/microsoft/typescript-go/blob/main/internal/ast/kind.go
// (Note: This code has been modified for this project.)
// =================================================================
#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    Unknown,
    EndOfFile,
    SingleLineCommentTrivia,
    MultiLineCommentTrivia,
    NewLineTrivia,
    WhitespaceTrivia,
    ConflictMarkerTrivia,
    NonTextFileMarkerTrivia,
    NumericLiteral,
    BigIntLiteral,
    StringLiteral,
    JsxText,
    JsxTextAllWhiteSpaces,
    RegularExpressionLiteral,
    NoSubstitutionTemplateLiteral,
    // Pseudo-literals
    TemplateHead,
    TemplateMiddle,
    TemplateTail,
    // Punctuation
    LeftBraceToken,                         // {
    RightBraceToken,                        // }
    LeftParenToken,                         // (
    RightParenToken,                        // )
    LeftBracketToken,                       // [
    RightBracketToken,                      // ]
    DotToken,                               // .
    DotDotDotToken,                         // ...
    SemicolonToken,                         // ;
    CommaToken,                             // ,
    QuestionDotToken,                       // ?.
    LessThanToken,                          // <
    LessThanSlashToken,                     // </
    GreaterThanToken,                       // >
    LessThanEqualsToken,                    // <=
    GreaterThanEqualsToken,                 // >=
    EqualsEqualsToken,                      // ==
    ExclamationEqualsToken,                 // !=
    EqualsEqualsEqualsToken,                // ===
    ExclamationEqualsEqualsToken,           // !==
    EqualsGreaterThanToken,                 // =>
    PlusToken,                              // +
    MinusToken,                             // -
    AsteriskToken,                          // *
    AsteriskAsteriskToken,                  // **
    SlashToken,                             // /
    PercentToken,                           // %
    PlusPlusToken,                          // ++
    MinusMinusToken,                        // --
    LessThanLessThanToken,                  // <<
    GreaterThanGreaterThanToken,            // >>
    GreaterThanGreaterThanGreaterThanToken, // >>>
    AmpersandToken,                         // &
    BarToken,                               // |
    CaretToken,                             // ^
    ExclamationToken,                       // !
    TildeToken,                             // ~
    AmpersandAmpersandToken,                // &&
    BarBarToken,                            // ||
    QuestionToken,                          // ?
    ColonToken,                             // :
    AtToken,                                // @
    QuestionQuestionToken,                  // ??
    /** Only the JSDoc scanner produces BacktickToken. The normal scanner produces NoSubstitutionTemplateLiteral and related kinds. */
    BacktickToken, // `
    /** Only the JSDoc scanner produces HashToken. The normal scanner produces PrivateIdentifier. */
    HashToken, // #
    // Assignments
    EqualsToken,                                  // =
    PlusEqualsToken,                              // +=
    MinusEqualsToken,                             // -=
    AsteriskEqualsToken,                          // *=
    AsteriskAsteriskEqualsToken,                  // **=
    SlashEqualsToken,                             // /=
    PercentEqualsToken,                           // %=
    LessThanLessThanEqualsToken,                  // <<=
    GreaterThanGreaterThanEqualsToken,            // >>=
    GreaterThanGreaterThanGreaterThanEqualsToken, // >>>=
    AmpersandEqualsToken,                         // &=
    AmpersandAmpersandEqualsToken,                // &&=
    BarEqualsToken,                               // |=
    BarBarEqualsToken,                            // ||=
    QuestionQuestionEqualsToken,                  // ??=
    CaretEqualsToken,                             // ^=
    // Identifiers and PrivateIdentifier
    Identifier,
    PrivateIdentifier,
    JSDocCommentTextToken,
    // Reserved words
    BreakKeyword,      // break
    CaseKeyword,       // case
    CatchKeyword,      // catch
    ClassKeyword,      // class
    ConstKeyword,      // const
    ContinueKeyword,   // continue
    DebuggerKeyword,   // debugger
    DefaultKeyword,    // default
    DeleteKeyword,     // delete
    DoKeyword,         // do
    ElseKeyword,       // else
    EnumKeyword,       // enum
    ExportKeyword,     // export
    ExtendsKeyword,    // extends
    FalseKeyword,      // false
    FinallyKeyword,    // finally
    ForKeyword,        // for
    FunctionKeyword,   // function
    IfKeyword,         // if
    ImportKeyword,     // import
    InKeyword,         // in
    InstanceOfKeyword, //instance
    NewKeyword,        //new
    NullKeyword,       //null
    ReturnKeyword,     //return
    SuperKeyword,      //super
    SwitchKeyword,     //switch
    ThisKeyword,       //this
    ThrowKeyword,      //throw
    TrueKeyword,       //true
    TryKeyword,        //try
    TypeOfKeyword,     //typeof
    VarKeyword,        //var
    VoidKeyword,       //void
    WhileKeyword,      //while
    WithKeyword,       //with
    // Strict mode reserved words
    ImplementsKeyword, //implements
    InterfaceKeyword,  //interface
    LetKeyword,        //let
    PackageKeyword,    //package
    PrivateKeyword,    //private
    ProtectedKeyword,  //protected
    PublicKeyword,     //public
    StaticKeyword,     //static
    YieldKeyword,      //yield
    // Contextual keywords
    AbstractKeyword,    //abstract
    AccessorKeyword,    //accessor
    AsKeyword,          //as
    AssertsKeyword,     //asserts
    AssertKeyword,      //assert
    AnyKeyword,         //any
    AsyncKeyword,       //async
    AwaitKeyword,       //await
    BooleanKeyword,     //Boolean
    ConstructorKeyword, //constructor
    DeclareKeyword,     //declare
    GetKeyword,         //get
    ImmediateKeyword,   //immediate
    InferKeyword,       //infer
    IntrinsicKeyword,   //intrinsic
    IsKeyword,          //is
    KeyOfKeyword,       //keyof
    ModuleKeyword,      //module
    NamespaceKeyword,   //namespace
    NeverKeyword,       //never
    OutKeyword,         //out
    ReadonlyKeyword,    //readonly
    RequireKeyword,     //require
    NumberKeyword,      //number
    ObjectKeyword,      //object
    SatisfiesKeyword,   //satisfies
    SetKeyword,         //set
    StringKeyword,      // String
    SymbolKeyword,      //symbol
    TypeKeyword,        //type
    UndefinedKeyword,   //undefined
    UniqueKeyword,      //unique
    UnknownKeyword,     //unknown
    UsingKeyword,       //using
    FromKeyword,        //from
    GlobalKeyword,      //global
    BigIntKeyword,      //BigInt
    OverrideKeyword,    //override
    OfKeyword,          // of, LastKeyword and LastToken and LastContextualKeyword
    // Parse tree nodes
    // Names
    QualifiedName,
    ComputedPropertyName,
    // Signature elements
    TypeParameter,
    Parameter,
    Decorator,
    // TypeMember
    PropertySignature,
    PropertyDeclaration,
    MethodSignature,
    MethodDeclaration,
    ClassStaticBlockDeclaration,
    Constructor,
    GetAccessor,
    SetAccessor,
    CallSignature,
    ConstructSignature,
    IndexSignature,
    // Type
    TypePredicate,
    TypeReference,
    FunctionType,
    ConstructorType,
    TypeQuery,
    TypeLiteral,
    ArrayType,
    TupleType,
    OptionalType,
    RestType,
    UnionType,
    IntersectionType,
    ConditionalType,
    InferType,
    ParenthesizedType,
    ThisType,
    TypeOperator,
    IndexedAccessType,
    MappedType,
    LiteralType,
    NamedTupleMember,
    TemplateLiteralType,
    TemplateLiteralTypeSpan,
    ImportType,
    // Binding patterns
    ObjectBindingPattern,
    ArrayBindingPattern,
    BindingElement,
    // Expression
    ArrayLiteralExpression,
    ObjectLiteralExpression,
    PropertyAccessExpression,
    ElementAccessExpression,
    CallExpression,
    NewExpression,
    TaggedTemplateExpression,
    TypeAssertionExpression,
    ParenthesizedExpression,
    FunctionExpression,
    ArrowFunction,
    DeleteExpression,
    TypeOfExpression,
    VoidExpression,
    AwaitExpression,
    PrefixUnaryExpression,
    PostfixUnaryExpression,
    BinaryExpression,
    ConditionalExpression,
    TemplateExpression,
    YieldExpression,
    SpreadElement,
    ClassExpression,
    OmittedExpression,
    ExpressionWithTypeArguments,
    AsExpression,
    NonNullExpression,
    MetaProperty,
    SyntheticExpression,
    SatisfiesExpression,
    // Misc
    TemplateSpan,
    SemicolonClassElement,
    // Element
    Block,
    EmptyStatement,
    VariableStatement,
    ExpressionStatement,
    IfStatement,
    DoStatement,
    WhileStatement,
    ForStatement,
    ForInStatement,
    ForOfStatement,
    ContinueStatement,
    BreakStatement,
    ReturnStatement,
    WithStatement,
    SwitchStatement,
    LabeledStatement,
    ThrowStatement,
    TryStatement,
    DebuggerStatement,
    VariableDeclaration,
    VariableDeclarationList,
    FunctionDeclaration,
    ClassDeclaration,
    InterfaceDeclaration,
    TypeAliasDeclaration,
    EnumDeclaration,
    ModuleDeclaration,
    ModuleBlock,
    CaseBlock,
    NamespaceExportDeclaration,
    ImportEqualsDeclaration,
    ImportDeclaration,
    ImportClause,
    NamespaceImport,
    NamedImports,
    ImportSpecifier,
    ExportAssignment,
    ExportDeclaration,
    NamedExports,
    NamespaceExport,
    ExportSpecifier,
    MissingDeclaration,
    // Module references
    ExternalModuleReference,
    // JSX
    JsxElement,
    JsxSelfClosingElement,
    JsxOpeningElement,
    JsxClosingElement,
    JsxFragment,
    JsxOpeningFragment,
    JsxClosingFragment,
    JsxAttribute,
    JsxAttributes,
    JsxSpreadAttribute,
    JsxExpression,
    JsxNamespacedName,
    // Clauses
    CaseClause,
    DefaultClause,
    HeritageClause,
    CatchClause,
    // Import attributes
    ImportAttributes,
    ImportAttribute,
    // Property assignments
    PropertyAssignment,
    ShorthandPropertyAssignment,
    SpreadAssignment,
    // Enum
    EnumMember,
    // Top-level nodes
    SourceFile,
    Bundle,
    // JSDoc nodes
    JSDocTypeExpression,
    JSDocNameReference,
    JSDocMemberName, // C#p
    JSDocAllType,    // The * type
    JSDocNullableType,
    JSDocNonNullableType,
    JSDocOptionalType,
    JSDocVariadicType,
    JSDoc,
    JSDocText,
    JSDocTypeLiteral,
    JSDocSignature,
    JSDocLink,
    JSDocLinkCode,
    JSDocLinkPlain,
    JSDocTag,
    JSDocAugmentsTag,
    JSDocImplementsTag,
    JSDocDeprecatedTag,
    JSDocPublicTag,
    JSDocPrivateTag,
    JSDocProtectedTag,
    JSDocReadonlyTag,
    JSDocOverrideTag,
    JSDocCallbackTag,
    JSDocOverloadTag,
    JSDocParameterTag,
    JSDocReturnTag,
    JSDocThisTag,
    JSDocTypeTag,
    JSDocTemplateTag,
    JSDocTypedefTag,
    JSDocSeeTag,
    JSDocPropertyTag,
    JSDocSatisfiesTag,
    JSDocImportTag,
    // Synthesized list
    SyntaxList,
    // Reparsed JS nodes
    JSTypeAliasDeclaration,
    JSExportAssignment,
    CommonJSExport,
    JSImportDeclaration,
    // Transformation nodes
    NotEmittedStatement,
    PartiallyEmittedExpression,
    CommaListExpression,
    SyntheticReferenceExpression,
    NotEmittedTypeElement,
    // Enum value count
    Count,
    // Markers
    FirstAssignment,         // = EqualsToken
    LastAssignment,          // = CaretEqualsToken
    FirstCompoundAssignment, // = PlusEqualsToken
    LastCompoundAssignment,  // = CaretEqualsToken
    FirstReservedWord,       // = BreakKeyword
    LastReservedWord,        // = WithKeyword
    FirstKeyword,            // = BreakKeyword
    LastKeyword,             // = OfKeyword
    FirstFutureReservedWord, // = ImplementsKeyword
    LastFutureReservedWord,  // = YieldKeyword
    FirstTypeNode,           // = TypePredicate
    LastTypeNode,            // = ImportType
    FirstPunctuation,        // = LeftBraceToken
    LastPunctuation,         // = CaretEqualsToken
    FirstToken,              // = Unknown
    LastToken,               // = LastKeyword
    FirstLiteralToken,       // = NumericLiteral
    LastLiteralToken,        // = NoSubstitutionTemplateLiteral
    FirstTemplateToken,      // = NoSubstitutionTemplateLiteral
    LastTemplateToken,       // = TemplateTail
    FirstBinaryOperator,     // = LessThanToken
    LastBinaryOperator,      // = CaretEqualsToken
    FirstStatement,          // = VariableStatement
    LastStatement,           // = DebuggerStatement
    FirstNode,               // = QualifiedName
    FirstJSDocNode,          // = JSDocTypeExpression
    LastJSDocNode,           // = JSDocImportTag
    FirstJSDocTagNode,       // = JSDocTag
    LastJSDocTagNode,        // = JSDocImportTag
    FirstContextualKeyword,  // = AbstractKeyword
    LastContextualKeyword,   // = OfKeyword
    Comment,                 // = SingleLineCommentTrivia | KindMultiLineCommentTrivia
    FirstTriviaToken,        // = SingleLineCommentTrivia
    LastTriviaToken,         // = ConflictMarkerTrivia
}

pub struct Token<'a> {
    token_type: TokenType,
    line: usize,
    loc: usize,
    lexeme: &'a str,
    literal_value: &'a str,
}

impl<'a> Token<'a> {
    fn new(
        token_type: TokenType,
        line: usize,
        loc: usize,
        lexeme: &'a str,
        literal_value: &'a str,
    ) -> Self {
        Self {
            token_type,
            line,
            loc,
            lexeme,
            literal_value,
        }
    }
}
