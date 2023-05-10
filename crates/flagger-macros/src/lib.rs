use std::collections::{HashMap};

use proc_macro::{TokenStream};
use syn::{DeriveInput, Fields, Expr, Ident, Lit, BinOp};
use quote::*;

#[derive(Clone)]
enum FlagValue {
    Expr(Expr),
    Value(u128),
    Implicit
}

#[proc_macro_attribute]
pub fn flags(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();

    let data_enum = match ast.data {
        syn::Data::Enum(data) => data,
        _ => panic!("Flags macro only works on enums")
    };

    let mut unprocessed_flags = Vec::<(Ident, FlagValue)>::new();
    let mut processed_flags = HashMap::<Ident, FlagValue>::new();

    for variant in data_enum.variants {
        match variant.fields {
            Fields::Unit => (),
            _ => panic!("Variants with fields are not allowed")
        }

        let flag_value = match variant.discriminant {
            Some(expr) => FlagValue::Expr(expr.1),
            None => FlagValue::Implicit
        };

        unprocessed_flags.push((variant.ident, flag_value))
    }

    let mut processed_any = true;
    while processed_any {
        processed_any = false;
        
        let mut i = 0usize;
        while i < unprocessed_flags.len() {
            let (ident, value) = &unprocessed_flags[i as usize];

            let processed_flag_value = process_discriminant(ident, value, &processed_flags);

            match processed_flag_value {
                FlagValue::Value(_) => {
                    processed_flags.insert(ident.clone(), processed_flag_value);
                    unprocessed_flags.remove(i as usize);
                    processed_any = true;
                }
                _ => {
                    i += 1;
                }
            }
        }
    }

    let highest_bit = 31;

    if unprocessed_flags.len() > 0 {
        let (ident, value) = &unprocessed_flags[0];
        match value {
            FlagValue::Value(_) => (),
            _ => panic!("Unable to determine value for \"{ident}\"")
        }
    }
    
    let variants: Vec<_> = processed_flags.into_iter()
        .map(|(ident, value)| {
            (ident, match value {
                FlagValue::Value(value) => value as u32,
                _ => unreachable!()
            })
        })
        .map(|(ident, value)| {
            quote! {
                const #ident: Self = Self(#value);
            }
        })
        .collect();

    let representation = match highest_bit {
        0..=7 => quote! { u8 },
        8..=15 => quote! { u16 },
        16..=31 => quote! { u32 },
        32..=63 => quote! { u64 },
        64..=127 => quote! { u128 },
        _ => panic!("Cannot repr flags of this size")
    };

    let name = ast.ident;
    let visibility = ast.vis;

    quote! {
        #[derive(Clone, Copy, Eq, PartialEq)]
        #visibility struct #name (#representation);

        impl Flags for #name {
            type Representation = #representation;
            const None: Self = Self(0);
            const All: Self = Self(#representation::MAX);
        }

        impl std::convert::From<#name> for #representation {
            fn from(value: #name) -> Self {
                value.0
            }
        }

        #[allow(non_upper_case_globals)]
        impl #name {
            #(#variants)*
        }

        impl std::ops::BitAnd for #name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitAndAssign for #name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl std::ops::BitOr for #name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitOrAssign for #name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl std::ops::BitXor for #name {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXorAssign for #name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }

        impl std::ops::Not for #name {
            type Output = Self;
            fn not(self) -> Self{
                Self(!self.0)
            }
        }
    }.into()
}

fn process_discriminant(ident: &Ident, value: &FlagValue, processed_flags: &HashMap<Ident, FlagValue>) -> FlagValue {
    match value {
        FlagValue::Expr(expr) => {
            match parse_discriminant(ident, expr, processed_flags){
                Some(value) => FlagValue::Value(value),
                None => value.clone()
            }
        },
        _ => value.clone()
    }
}

fn parse_discriminant(ident: &Ident, expr: &Expr, processed_flags: &HashMap<Ident, FlagValue>) -> Option<u128> {
    match expr {
        Expr::Lit(expr_lit) => {
            match &expr_lit.lit {
                Lit::Int(lit_int) => {
                    Some(lit_int.base10_digits().parse::<u128>().unwrap())
                },
                _ => panic!("Invalid discriminant for {ident}")
            }
        },
        Expr::Path(expr_path) => {
            let segments = &expr_path.path.segments;
            if segments.len() != 2 {
                panic!("Invalid discriminant for {ident}")
            }

            if segments[0].ident != *ident && segments[0].ident.to_string() != "Self" {
                panic!("Invalid discriminant for {ident}")
            }

            match processed_flags.get(&segments[1].ident) {
                Some(flag_value) => match flag_value {
                    FlagValue::Value(value) => Some(*value),
                    _ => panic!("Invalid discriminant for {ident}")
                },
                None => None
            }
        },
        Expr::Binary(binary) => {
            let lhs = parse_discriminant(ident, &*binary.left, processed_flags)?;
            let rhs = parse_discriminant(ident, &*binary.right, processed_flags)?;
            Some(match binary.op {
                BinOp::BitAnd(_) => lhs & rhs,
                BinOp::BitOr(_) => lhs | rhs,
                BinOp::BitXor(_) => lhs ^ rhs,
                _ => panic!("Invalid discriminant for {ident}")
            })
        }
        _ => panic!("Invalid discriminant for {ident}")
    }
}