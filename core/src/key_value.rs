use crate::value::{Value, ValueExpr};
use crate::ParseRaw;
use proc_macro2::{TokenStream, TokenTree};
use quote::TokenStreamExt;
use std::marker::PhantomData;
use syn::parse::Parser;
use syn::parse::{self, Parse};

pub struct KeyValue<V> {
    key: Value<V>,
    value: ValueExpr<V>,
}

impl<V> KeyValue<V> {
    pub fn key(&self) -> &Value<V> {
        &self.key
    }

    pub fn value(&self) -> &ValueExpr<V> {
        &self.value
    }
}

pub struct KeyValueSeq<V> {
    key_values: Vec<KeyValue<V>>,
    _phantom: PhantomData<V>,
}

impl<V> KeyValueSeq<V> {
    pub fn key_values(&self) -> impl ExactSizeIterator<Item = &KeyValue<V>> {
        self.key_values.iter()
    }
}
impl<V> ParseRaw for KeyValueSeq<V>
where
    Value<V>: Parse,
    ValueExpr<V>: Parse,
{
    fn parse_raw(tokens: TokenStream) -> parse::Result<KeyValueSeq<V>> {
        let mut remainder = tokens;
        let mut key_values = Vec::new();
        while !remainder.is_empty() {
            let (key, tokens) = parse_key(remainder)?;
            let (value, tokens) = parse_value(tokens)?;
            remainder = tokens;
            key_values.push(KeyValue { key, value });
        }
        Ok(KeyValueSeq {
            key_values,
            _phantom: PhantomData,
        })
    }
}

fn parse_key<V>(tokens: TokenStream) -> parse::Result<(Value<V>, TokenStream)>
where
    Value<V>: Parse,
{
    let mut it = tokens.into_iter();
    let mut key = TokenStream::new();
    while let Some(tt) = it.next() {
        if let TokenTree::Punct(p) = &tt {
            if p.as_char() == ':' {
                break;
            }
        }
        key.append(tt);
    }
    Ok((Value::parse.parse2(key)?, it.collect()))
}

fn parse_value<V>(tokens: TokenStream) -> parse::Result<(ValueExpr<V>, TokenStream)>
where
    ValueExpr<V>: Parse,
{
    let mut it = tokens.into_iter();
    let mut value = TokenStream::new();
    while let Some(tt) = it.next() {
        if let TokenTree::Punct(p) = &tt {
            if p.as_char() == ',' {
                break;
            }
        }
        value.append(tt);
    }
    Ok((ValueExpr::parse.parse2(value)?, it.collect()))
}
