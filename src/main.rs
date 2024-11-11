use html_parser::{parse_html, HtmlElem, HtmlParseError};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> Result<(),  Box<dyn std::error::Error>> {
    let input = "<!DOCTYPE html>
                    <html>
                            <head> 
                                <br/> 
                                <label>Some text!</label>
                                <div>
                                 text
                                </div>
                            </head>
                            <body>
                            <br/>
                            <p> jnfdfjndf</p>
                            </body>  
                    </html>           
                        ";
    let parsed_html = parse_html(input)?;
    for child in &parsed_html {
            print_tree(child, "");
    }
    Ok(())
    // match parse_html(input) {
    //     Ok(elements) => {
    //         for child in &elements {
    //             print_tree(child, "");
    //         }
    //     }
    
    //     Err(e) => println!("Parsing failed: {}", e),
    // }
}

fn print_tree(root: &HtmlElem, indent: &str) {
    match root {
        HtmlElem::Tag { tag_name, children } => {
            println!("{} {}", indent, tag_name);
            if !children.is_empty() {
                let new_indent = format!("{}  ", indent);
                for child in children {
                    print_tree(child, &new_indent);
                }
            }
        }
        HtmlElem::Text(text) => {
            println!("{} ({})", indent, text);
        }
        HtmlElem::Documentation(doctype) => {
            println!("{}", doctype);
        }
    }
}
