# to-query generated AST

```
DeriveInput {
    attrs: [],
    vis: Visibility::Inherited,
    ident: Ident {
        ident: "Search",
        span: #0 bytes(69..75),
    },
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    data: Data::Struct {
        struct_token: Struct,
        fields: Fields::Named {
            brace_token: Brace,
            named: [
                Field {
                    attrs: [],
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(
                        Ident {
                            ident: "term",
                            span: #0 bytes(82..86),
                        },
                    ),
                    colon_token: Some(
                        Colon,
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "String",
                                        span: #0 bytes(88..94),
                                    },
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                },
                Comma,
                Field {
                    attrs: [],
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(
                        Ident {
                            ident: "page",
                            span: #0 bytes(100..104),
                        },
                    ),
                    colon_token: Some(
                        Colon,
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "u32",
                                        span: #0 bytes(106..109),
                                    },
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                },
                Comma,
                Field {
                    attrs: [
                        Attribute {
                            pound_token: Pound,
                            style: AttrStyle::Outer,
                            bracket_token: Bracket,
                            meta: Meta::NameValue {
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident {
                                                ident: "query_rename",
                                                span: #0 bytes(117..129),
                                            },
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                                eq_token: Eq,
                                value: Expr::Lit {
                                    attrs: [],
                                    lit: Lit::Str {
                                        token: "per_page",
                                    },
                                },
                            },
                        },
                    ],
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(
                        Ident {
                            ident: "per_page",
                            span: #0 bytes(148..156),
                        },
                    ),
                    colon_token: Some(
                        Colon,
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "Option",
                                        span: #0 bytes(158..164),
                                    },
                                    arguments: PathArguments::AngleBracketed {
                                        colon2_token: None,
                                        lt_token: Lt,
                                        args: [
                                            GenericArgument::Type(
                                                Type::Path {
                                                    qself: None,
                                                    path: Path {
                                                        leading_colon: None,
                                                        segments: [
                                                            PathSegment {
                                                                ident: Ident {
                                                                    ident: "u32",
                                                                    span: #0 bytes(165..168),
                                                                },
                                                                arguments: PathArguments::None,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                        ],
                                        gt_token: Gt,
                                    },
                                },
                            ],
                        },
                    },
                },
                Comma,
            ],
        },
        semi_token: None,
    },
}
```