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

pub struct FunctionNames {
    pub get_child_paths: Ident,
    pub get_child: Ident,
    pub get_children: Ident,

    pub get_child_owned: Ident,
    pub get_children_owned: Ident,

    pub get_located_child: Ident,
    pub get_located_children: Ident,

    pub get_located_child_owned: Ident,
    pub get_located_children_owned: Ident,
    pub into_located_children_owned: Ident,
}
impl FunctionNames {
    pub fn new(singular: &LitStr, plural: &LitStr) -> Self { Self {
        get_child_paths: create_ident(|x| format!{"get_{}_paths", x}, singular),
        get_child: create_ident(|x| format!{"get_{}", x}, singular),
        get_children: create_ident(|x| format!{"get_{}", x}, plural),

        get_child_owned: create_ident(|x| format!{"get_{}_owned", x}, singular),
        get_children_owned: create_ident(|x| format!{"get_{}_owned", x}, plural),

        get_located_child: create_ident(|x| format!{"get_located_{}", x}, singular),
        get_located_children: create_ident(|x| format!{"get_located_{}", x}, plural),

        get_located_child_owned: create_ident(|x| format!{"get_located_{}_owned", x}, singular),
        get_located_children_owned: create_ident(|x| format!{"get_located_{}_owned", x}, plural),
        into_located_children_owned: create_ident(|x| format!{"into_located_{}_owned", x}, plural),
    }}
}

fn create_ident<F: FnOnce(String) -> String>(formatter: F, lit: &LitStr) -> Ident
    { Ident::new(&formatter(lit.value()), lit.span()) }
