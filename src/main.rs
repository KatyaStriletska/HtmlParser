use pest_derive:: Parser;
use pest:: Parser;
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() {
    let example = "<!DOCTYPE html>
                        <html> 
                            <body>
                            </body>                    
                        </html>
                    ";
    let pasred_html_dec = Grammar:: parse(Rule::declaration,example);
    let pasred_html_struct = Grammar:: parse(Rule::html,example);

    println!("{:?}", pasred_html_dec);
    println!("{:?}", pasred_html_struct);

}  