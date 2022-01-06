use proc_macro::TokenStream;
use libxml::xpath::xml_xpath_compiles;

/// Method for validating whether the string is a feasible xpath
fn validate_xpath(mut xpath: String) -> std::result::Result<(), ()> {

    // Step one is to remove the surrounding quotation marks. 
    // If there are no quotation marks, then we fail
    let mut xpath_iter = xpath.chars();
    let first_char = xpath_iter.next();
    let last_char  = xpath_iter.next_back();
    if first_char != Some('"') || last_char != Some('"') {
        return Err(());
    }
    xpath = xpath_iter.collect();

    // Now we call our libxml binding to see if the xpath coompiles
    let compiles = xml_xpath_compiles(&xpath);
    if !compiles {
        Err(())
    } else {
        Ok(())
    }
}

/// We don't actually want to manipulate any token streams here. In reality, this isn't
/// even a macro. All we really want to do is have the chance to evaluate the xpath at 
/// compile time, and panic if we need to, so we'll convrt the token stream to a string, 
/// validate the path, and then pass the input token stream right back out.
#[proc_macro]
pub fn xpath(input: TokenStream) -> TokenStream {

    // If the xpath isn't syntactically valid, panic
    match validate_xpath(input.to_string()) {
        Ok(_) => input,
        Err(_) => {
            quote::quote! {
                compile_error!("Invalid XPath");
            }.into()
        }
    }
}

