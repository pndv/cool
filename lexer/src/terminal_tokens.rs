﻿use crate::tokens::{Token, CLOSE_CURL_TYPE, CLOSE_PAREN_TYPE, COMMA_TYPE, ELSE_TYPE, END_IF_TYPE, END_LOOP_TYPE, IN_TYPE, LOOP_TYPE, OF_TYPE, SEMI_COLON_TYPE, THEN_TYPE};

pub(crate) const TERMINATE_TOKEN_CLOSE_PAREN: [Token; 1] = [CLOSE_PAREN_TYPE];
pub(crate) const TERMINATE_TOKEN_SEMI_COLON: [Token; 1] = [SEMI_COLON_TYPE];
pub(crate) const TERMINATE_TOKEN_LOOP: [Token; 1] = [LOOP_TYPE];
pub(crate) const TERMINATE_TOKEN_END_LOOP: [Token; 1] = [END_LOOP_TYPE];
pub(crate) const TERMINATE_TOKEN_THEN: [Token; 1] = [THEN_TYPE];
pub(crate) const TERMINATE_TOKEN_ELSE: [Token; 1] = [ELSE_TYPE];
pub(crate) const TERMINATE_TOKEN_END_IF: [Token; 1] = [END_IF_TYPE];
pub(crate) const TERMINATE_TOKEN_OF: [Token; 1] = [OF_TYPE];
pub(crate) const TERMINATE_TOKENS_DISPATCH_FN_PARAMS: [Token; 2] = [COMMA_TYPE, CLOSE_PAREN_TYPE];
pub(crate) const TERMINATE_TOKENS_LET_INIT_EXPR: [Token; 2] = [COMMA_TYPE, IN_TYPE];
pub(crate) const TERMINATE_TOKENS_FEATURES: [Token; 2] = [SEMI_COLON_TYPE, CLOSE_CURL_TYPE];
pub(crate) const TERMINATE_TOKEN_FEATURE_METHOD_EXPR: [Token; 1] = [CLOSE_CURL_TYPE];
