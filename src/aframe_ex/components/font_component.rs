use aframers::components::core::ComponentValue;

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