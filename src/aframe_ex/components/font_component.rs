use aframers::components::core::ComponentValue;

#[derive(Debug, Copy, Clone)]
pub struct CustomFont(pub &'static str);
impl ComponentValue for CustomFont {
	fn component_name(&self) -> &str {
		"font"
	}
	fn component_value(&self) -> impl AsRef<str> {
		self.0
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Negate(pub bool);
impl ComponentValue for Negate {
	fn component_name(&self) -> &str {
		"negate"
	}
	fn component_value(&self) -> impl AsRef<str> {
		match self.0 {
			true => "true",
			false => "false",
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub enum StockFont {
	Roboto,
	AileronSemiBold,
	DejaVu,
	Exo2Bold,
	Exo2Semibold,
	KelsonSans,
	Monoid,
	MozillaVr,
	SourceCodePro,
}
impl AsRef<str> for StockFont {
	//noinspection SpellCheckingInspection
	fn as_ref(&self) -> &str {
		match self {
			StockFont::Roboto => "roboto",
			StockFont::AileronSemiBold => "aileronsemibold",
			StockFont::DejaVu => "dejavu",
			StockFont::Exo2Bold => "exo2bold",
			StockFont::Exo2Semibold => "exo2semibold",
			StockFont::KelsonSans => "kelsonsans",
			StockFont::Monoid => "monoid",
			StockFont::MozillaVr => "mozillavr",
			StockFont::SourceCodePro => "sourcecodepro",
		}
	}
}
impl ComponentValue for StockFont {
	fn component_name(&self) -> &str { "font" }
	fn component_value(&self) -> impl AsRef<str> { self }
}