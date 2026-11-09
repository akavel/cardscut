use easy_error::Error;
use pdfluent_lopdf::Document;
use std::path::PathBuf;

mod errctx;
use errctx::*;

const PDF: &str = "Ironsworn-Starforged-Assets-Sheets.pdf";

fn main() -> Result<(), Error> {
    // TODO: allow listing card names and picking a card by name
    let flags = xflags::parse_or_exit! {
        /// PDF file with the original cards. Should be downloaded from:
        /// https://tomkinpress.com/products/ironsworn-starforged-printable-asset-cards
        optional -i,--input path: PathBuf
        /// Output file; defaults to: "cards.pdf".
        optional -o,--output path: PathBuf
        /// List of 1-4 cards to pick. Format: page:index,... e.g.: "2:1,5:4,5:6" is:
        /// Starship, Bounty Hunter, Courier.
        required -c,--cards spec: String
    };

    let in_path = flags.input.unwrap_or_else(|| PathBuf::from(PDF));
    let out_path = flags.output.unwrap_or_else(|| PathBuf::from("cards.pdf"));
    let input = Document::load(&in_path).ctx(
        "loading PDF (REMEMBER: make sure to download: https://tomkinpress.com/products/ironsworn-starforged-printable-asset-cards ?)")?;

    let mut result = pdf_manip::pages::extract_pages(&input, &[2]).ctx("extracting pages")?;
    result.save(&out_path).ctx("saving PDF")?;

    println!("Hello, world!");
    Ok(())
}
