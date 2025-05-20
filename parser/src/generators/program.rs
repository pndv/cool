use crate::generators::class::gen_class;
use crate::model::program::ParseProgram;
use lexer::iter::token::{BaseTokenIter, BufferedTokenIter, TokenIter};
use lexer::model::constants::SEMI_COLON_TYPE;
use std::fs::File;

pub(crate) fn gen_program_from_file(file: File) -> Result<ParseProgram, String> {
    let mut token_iter: TokenIter = TokenIter::from(file);
    gen_program(&mut token_iter)
}

/// Program is a list of semicolon separated classes

fn gen_program(iter: &mut TokenIter) -> Result<ParseProgram, String> {
    let mut program: ParseProgram = ParseProgram::new();
    let mut errors = String::new();

    while iter.has_next() {
        let program_tokens = iter.collect_till(&SEMI_COLON_TYPE);

        if program_tokens.is_empty() {
            continue;
        }

        let mut buffered_iter = BufferedTokenIter::from(program_tokens);

        match gen_class(&mut buffered_iter) {
            Ok(class) => program.add_class(class),
            Err(e) => errors.push_str(&e),
        }

        iter.consume_required(&SEMI_COLON_TYPE)?;
    }

    if errors.is_empty() {
        Ok(program)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod program_test {
    use super::*;
    use crate::model::expressions::Expression;
    use crate::model::feature::{Attribute, Method, ParseFeature};
    use std::collections::HashMap;
    use std::ffi::OsStr;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_partial_dispatch() {
        let file = File::open("../test_resources/programs/partial_dispatch.cl").expect("Cannot open file");
        let program_result = gen_program_from_file(file);
        assert!(program_result.is_ok());
        let program = program_result.unwrap();
        println!("{:#?}", program);
        
    }
    
    #[test]
    fn test_single_program() {
        let file = File::open("../test_resources/programs/cool.cl").expect("Cannot open file");
        let program_result = gen_program_from_file(file);
        assert!(program_result.is_ok());
    }

    #[test]
    fn test_arith() {
        let file = File::open("../test_resources/programs/arith.cl").expect("Cannot open file");
        let program_result = gen_program_from_file(file);
        assert!(program_result.is_ok());
        let program = program_result.unwrap();
        assert!(program.classes().len() > 1);
        assert_eq!(program.classes().len(), 7);
        println!("{:#?}", program.classes().len());
    }

    #[test]
    fn test_lam() {
        let file = File::open("../test_resources/programs/lam.cl").expect("Cannot open file");
        let program_result = gen_program_from_file(file);
        assert!(program_result.is_ok());
        let program = program_result.unwrap();
        assert!(program.classes().len() > 1);
        assert_eq!(program.classes().len(), 11);
        println!("{:#?}", program.classes().len());
    }

    #[test]
    fn test_primes() {
        let file = File::open("../test_resources/programs/primes.cl").expect("Cannot open file");
        let program_result = gen_program_from_file(file);
        assert!(program_result.is_ok());
        let program = program_result.unwrap();
        assert_eq!(program.classes().len(), 1);
        println!("{:#?}", program.classes().len());
    }

    #[test]
    #[should_panic]
    fn test_single_program_fail() {
        let file = File::open("../../../test_resources/cool_bad.cl").expect("Cannot open file");
        let program = gen_program_from_file(file);
        assert!(program.is_err());
    }

    #[test]
    fn test_all_programs() {
        let filter_extn = OsStr::new("cl");
        let dir = Path::new("../test_resources/programs");
        let entries = fs::read_dir(dir).expect("Cannot open dir");

        let mut has_partials = false;
        let mut file_pass_map: HashMap<String, bool> = HashMap::new();
        
        for entry in entries {
            if entry.is_err() {
                continue;
            }
            let path = entry.unwrap().path();
            if path.extension() != Some(filter_extn) {
                continue;
            }
            print!("==== Processing: {:#?}", path);
            let file = File::open(path.clone()).expect("Cannot open file");
            let mut token_iter = TokenIter::from(file);
            let result = gen_program(&mut token_iter);
            println!(" = {:#?}", result.is_ok());
            assert!(result.is_ok());

            let pgm = result.unwrap();
            
            for class in pgm.classes() {
                if class.features.is_none() {
                    continue
                }

                for feature in class.features.as_ref().unwrap() {
                    let opt_expr = match feature {
                        ParseFeature::Attribute { attribute: Attribute {name, return_type, expr} } => expr.as_ref(),
                        ParseFeature::Method { method: Method {name, formals , return_type, expr} } => Some(expr),
                    };

                    if opt_expr.is_none() {
                        continue;
                    }
                    let expr = opt_expr.unwrap();
                    if contains_partial(&path, class.name.get_name().as_str(),  expr) {
                        has_partials = true;
                    }
                }
            }
            
            file_pass_map.insert(path.to_str().unwrap().to_string(), has_partials);
        }

        assert!(!has_partials);
        println!("{:#?}", dir);
        println!("{:#?}", file_pass_map);
    }

    fn contains_partial(file_path: &Path, class_name: &str, expression: &Expression) -> bool {
        match expression {
            Expression::PartialAssign { .. } |
            Expression::PartialDispatch { .. } |
            Expression::PartialCastDispatch { .. } |
            Expression::PartialBinary { .. } => {
                println!("File:{:#?} | Class {} |  {:#?}", file_path, class_name, expression);
                true 
            },

            Expression::IdentExpr { .. } |
            Expression::IntExpr { .. } |
            Expression::BoolExpr { .. } |
            Expression::StringExpr { .. } |
            Expression::SelfTypeExpr {.. } |
            Expression::SelfExpr |
            Expression::New { .. } => false,

            Expression::Dispatch { calling_expr, cast_type, fn_name, param_list } => {
                contains_partial(file_path, class_name, calling_expr) || param_list.iter().any(|e| contains_partial(file_path, class_name, e))
            }
            Expression::Conditional { predicate, then_expr, else_expr } => {
                contains_partial(file_path, class_name, predicate) || contains_partial(file_path, class_name, then_expr) || contains_partial(file_path, class_name, else_expr)
            }
            Expression::Loop { predicate, body } => {
                contains_partial(file_path, class_name, predicate) || contains_partial(file_path, class_name, body)
            }
            Expression::Case { switch_expression, branches } => {
                contains_partial(file_path, class_name, switch_expression) || branches.iter().any(|branch| contains_partial(file_path, class_name, &branch.expr))
            }
            Expression::Block { expr_list } => {
                expr_list.iter().any(|e| contains_partial(file_path, class_name, e))
            }
            Expression::Let { let_init, in_expr } => {
                let_init.iter().any(|init| init.expr.as_ref().map_or(false, |e| contains_partial(file_path, class_name, e))) || contains_partial(file_path, class_name, in_expr)
            }
            Expression::Plus { left, right } |
            Expression::Minus { left, right } |
            Expression::Multiply { left, right } |
            Expression::Divide { left, right } |
            Expression::LessThan { left, right } |
            Expression::Equal { left, right } |
            Expression::LessThanOrEqual { left, right } => {
                contains_partial(file_path, class_name, left) || contains_partial(file_path, class_name, right)
            }
            
            Expression::Assign {expr, .. } |
            Expression::Negate { expr } |
            Expression::IsVoid { expr } |
            Expression::Not { expr } => contains_partial(file_path, class_name, expr),
        }
    }
}
