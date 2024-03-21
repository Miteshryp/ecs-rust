use syn::{
    parse::Parse, token::Comma, Ident, LitInt
};

// Helper struct for Variadic Generics implementation (learned from bevy)
pub(crate) struct AllTuples {
    pub(crate) macro_caller: Ident,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) idents: Vec<Ident>
}

impl Parse for AllTuples {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let caller = input.parse::<Ident>()?;
        input.parse::<Comma>()?;

        let start: usize = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Comma>()?;

        let end: usize = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Comma>()?;
        
        // At least one identifier is must
        let mut ident_vec: Vec<Ident> = vec![input.parse::<Ident>()?];

        while input.parse::<Comma>().is_ok() {
            ident_vec.push(input.parse::<Ident>()?);
        }

        Ok(Self {
            macro_caller: caller,
            start,
            end,
            idents: ident_vec,
        })
    }
}
