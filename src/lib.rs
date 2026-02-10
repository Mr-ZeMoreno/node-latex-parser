use napi::Error;
use napi_derive::napi;

#[napi]
pub fn latex_to_ast(latex: String) -> Result<std::string::String, napi::Error> {
  match latex2mathml::latex_to_ast(&latex) {
    Ok(ast) => Ok(format!("{:#?}", ast)),
    Err(e) => Err(Error::from_reason(e.to_string())),
  }
}

#[napi(string_enum)]
pub enum DisplayStyle {
  Block,
  Inline,
}

impl From<latex2mathml::DisplayStyle> for DisplayStyle {
  fn from(native: latex2mathml::DisplayStyle) -> Self {
    match native {
      latex2mathml::DisplayStyle::Block => DisplayStyle::Block,
      latex2mathml::DisplayStyle::Inline => DisplayStyle::Inline,
    }
  }
}

impl From<DisplayStyle> for latex2mathml::DisplayStyle {
  fn from(wrapper: DisplayStyle) -> Self {
    match wrapper {
      DisplayStyle::Block => latex2mathml::DisplayStyle::Block,
      DisplayStyle::Inline => latex2mathml::DisplayStyle::Inline,
    }
  }
}

#[napi]
pub fn latex_to_mathml(
  latex: String,
  display: DisplayStyle,
) -> Result<std::string::String, napi::Error> {
  let native_display: latex2mathml::DisplayStyle = display.into();

  match latex2mathml::latex_to_mathml(&latex, native_display) {
    Ok(string) => Ok(string),
    Err(e) => Err(Error::from_reason(e.to_string())),
  }
}
