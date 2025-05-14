use parser::model::feature::{Attribute, Method, ParseFeature};
use parser::model::formal::Formal;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Default)]
pub struct FeatureNode {
  pub(crate) name: String,
  pub(crate) param_type_map: Option<Vec<FormalNode>>, // if None, it's an Attribute; otherwise, it's a Method
  pub(crate) feature_type: String,
}

impl From<&ParseFeature> for FeatureNode {
  fn from(value: &ParseFeature) -> Self {
    match value {
      ParseFeature::Attribute { attribute: Attribute { name, return_type, expr } } => {
        Self { name: name.get_name(), param_type_map: None, feature_type: return_type.get_name() }
      }
      ParseFeature::Method { method: Method { name, formals, return_type, expr } } => {
        let params = match formals {
          None => vec![],
          Some(params) => params.iter().map(|param| {
            FormalNode::from(param)
          }).collect()
        };
        
        Self { name: name.get_name(), param_type_map: Some(params), feature_type: return_type.get_name() }
      }
    }
  }
}
impl Display for FeatureNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &self.param_type_map {
      None => write!(f, "[FEATURE][ATTRIBUTE] {}:{}", self.name, self.feature_type),
      Some(params) => {
        let param_strs: Vec<String> = params.iter().map(|p| format!("{}:{}", p.name, p.formal_type)).collect();
        let param_str = param_strs.join(", ");
        write!(f, "[FEATURE][METHOD] {} ({}):{}", self.name, param_str, self.feature_type)
      }
    }
  }
}



#[derive(Debug, Clone, Default)]
pub struct FormalNode {
  pub(crate) name: String,
  pub(crate) formal_type: String,
}

impl From<&Formal> for FormalNode {
  fn from(value: &Formal) -> Self {
    match value {
      Formal { formal_name, formal_type } => Self { name: formal_name.get_name(), formal_type: formal_type.get_name() }
    }
  }
}

impl Display for FormalNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {  
    write!(f, "[IDENT] {}:{}", self.name.clone(), self.formal_type.clone())
  }
}
