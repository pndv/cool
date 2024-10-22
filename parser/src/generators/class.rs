use crate::generators::feature::gen_features;
use crate::model::class::Class;
use crate::model::feature::Feature;
use crate::model::Type;
use lexer::iter::token::{BaseTokenIter, BufferedTokenIter};
use lexer::model::constants::{
  CLASS_TYPE, CLOSE_CURL_TYPE, IDENT_TYPE, INHERITS_TYPE, OPEN_CURL_TYPE,
};
use lexer::model::token::Token;

pub(super) fn gen_class(iter: &mut BufferedTokenIter) -> Result<Class, String> {
    let mut errors = String::new();
    iter.consume_required(&CLASS_TYPE)?;

    let Token::Ident {
        value,
        line_num,
        line_pos,
    } = iter.get_required(&IDENT_TYPE)?
    else {
        unreachable!()
    };
    let class_type: Type = Type::from(value);

    let parent_type: Option<Type> = if iter.peek_eq(&INHERITS_TYPE) {
        match iter.consume_required(&INHERITS_TYPE) {
            Ok(()) => (),
            Err(e) => errors.push_str(&e),
        };

        let Token::Ident { value, .. } = iter.get_required(&IDENT_TYPE)? else {
            unreachable!()
        };

        let inherits_from: Type = Type::from(value);
        Some(inherits_from)
    } else {
        None
    };

    iter.consume_required(&OPEN_CURL_TYPE)?;

    let features: Option<Vec<Feature>> = if iter.peek_eq(&CLOSE_CURL_TYPE) {
        None
    } else {
        let mut feature_iter = iter.gen_iter_till(&CLOSE_CURL_TYPE);
        let gen_features = gen_features(&mut feature_iter)?;
        gen_features
    };

    match iter.consume_required(&CLOSE_CURL_TYPE) {
        Ok(()) => (),
        Err(e) => errors.push_str(&e),
    };

    if errors.is_empty() {
        Ok(Class::new(
            class_type,
            parent_type,
            features,
            line_num,
            line_pos,
        ))
    } else {
        Err(errors)
    }
}
