use rust_utils::hyperlink::HyperLink;

fn main() {
    let hyperlink =
        HyperLink::new("Kudos to Nick Larsen for the code! Follow him on GitHub Sponsors!", "https://github.com/sponsors/NickLarsenNZ").expect("valid construction");

    println!("{hyperlink}");
}
