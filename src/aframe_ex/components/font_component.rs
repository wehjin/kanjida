use aframers::components::core::ComponentAttribute;

#[derive(Debug, Copy, Clone)]
pub struct CustomFont(pub &'static str);
impl ComponentAttribute for CustomFont {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		"font"
	}
	fn as_attribute_str(&self) -> impl AsRef<str> {
		self.0
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Negate(pub bool);
impl ComponentAttribute for Negate {
	fn as_attribute_name(&self) -> impl AsRef<str> {
		"negate"
	}
	fn as_attribute_str(&self) -> impl AsRef<str> {
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
impl ComponentAttribute for StockFont {
	fn as_attribute_name(&self) -> impl AsRef<str> { "font" }
	fn as_attribute_str(&self) -> impl AsRef<str> { self }
}