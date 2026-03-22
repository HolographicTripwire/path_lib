use syn::{Ident, LitStr};

// pub struct SingularAndPluralChildLit {
//     singular: LitStr,
//     comma: Comma,
//     plural: LitStr
// }
// impl SingularAndPluralChildLit {
//     fn get_function_names(&self) -> FunctionNames {
//         FunctionNames::new(&self.singular, &self.plural)
//     }
// }

// impl Parse for SingularAndPluralChildLit {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        
//     }
// }

pub struct SingularSnakecaseIdents {
    pub get_child_paths: Ident,
    pub get_joined_child_paths: Ident,
    pub get_child: Ident,
    pub get_child_owned: Ident,
    pub get_located_child: Ident,
    pub get_located_child_owned: Ident,
}
impl SingularSnakecaseIdents {
    pub fn new(singular_snakecase: &LitStr) -> Self { Self {
        get_child_paths: create_ident(|x| format!{"get_{}_paths", x}, singular_snakecase),
        get_joined_child_paths: create_ident(|x| format!{"get_joined_{}_paths", x}, singular_snakecase),
        get_child: create_ident(|x| format!{"get_{}", x}, singular_snakecase),
        get_child_owned: create_ident(|x| format!{"get_{}_owned", x}, singular_snakecase),
        get_located_child: create_ident(|x| format!{"get_located_{}", x}, singular_snakecase),
        get_located_child_owned: create_ident(|x| format!{"get_located_{}_owned", x}, singular_snakecase),
    }}
}

pub struct PluralSnakecaseIdents {
    pub get_children: Ident,
    pub get_children_owned: Ident,
    pub get_located_children: Ident,
    pub get_located_children_owned: Ident,
    pub into_located_children_owned: Ident,
}
impl PluralSnakecaseIdents {
    pub fn new(plural_snakecase: &LitStr) -> Self { Self {
        get_children: create_ident(|x| format!{"get_{}", x}, plural_snakecase),
        get_children_owned: create_ident(|x| format!{"get_{}_owned", x}, plural_snakecase),
        get_located_children: create_ident(|x| format!{"get_located_{}", x}, plural_snakecase),
        get_located_children_owned: create_ident(|x| format!{"get_located_{}_owned", x}, plural_snakecase),
        into_located_children_owned: create_ident(|x| format!{"into_located_{}_owned", x}, plural_snakecase),
    }}
}

pub struct PluralTitlecaseIdents {
    pub parent_of_children: Ident,
    pub located_parent_of_children: Ident,
    pub owned_located_parent_of_children: Ident,
}
impl PluralTitlecaseIdents {
    pub fn new(plural_titlecase: &LitStr) -> Self { Self {
        parent_of_children: create_ident(|x| format!{"ParentOf{}", x}, plural_titlecase),
        located_parent_of_children: create_ident(|x| format!{"LocatedParentOf{}", x}, plural_titlecase),
        owned_located_parent_of_children: create_ident(|x| format!{"OwnedLocatedParentOf{}", x}, plural_titlecase),
    }}
}

fn create_ident<F: FnOnce(String) -> String>(formatter: F, lit: &LitStr) -> Ident
    { Ident::new(&formatter(lit.value()), lit.span()) }
