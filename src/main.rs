use pest::Parser;
use pest_derive::Parser;
use html_parser::{parse_html, HtmlElem};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main()  -> Result<(), Box<dyn std::error::Error>>{
    let input = "<!DOCTYPE html>
                    <html>
                            <head> 
                                <br/> 
                                <lab>Some text!</label>
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
    match parse_html(input) {
            Ok(elements) => print_html_dom(&elements),
            Err(e) => println!("Parsing failed: {}", e),
                        }
    
    Ok(())
}
fn print_html_dom(elements: &Vec<HtmlElem>){
    // if elements[0] == HtmlElem::Documentation(()) &&{

    // }
    println!("{:?}", elements[0]);
    println!("{:?}", elements[1]);
    
    // for elem in elements[1] {
    //     println!("{}", elem);
    // }
}
fn print_tree(html: &HtmlElem){
   
}
