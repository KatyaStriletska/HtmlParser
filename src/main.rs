use pest::Parser;
use pest_derive::Parser;
use html_parser::parse_html;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main()  -> Result<(), Box<dyn std::error::Error>>{
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
    match parse_html(input) {
            Ok(_) => println!("Parsing successful!"),
            Err(e) => println!("Parsing failed: {}", e),
                        }
    
    Ok(())
}
