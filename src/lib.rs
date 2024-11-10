use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Error, Debug)]
pub enum HtmlParseError{
    #[error("Invalid html")]
    ErrorHtmlStructure, 
    #[error("unknown data store error")]
    Unknown,
}
#[derive(Debug)]
enum HtmlElem
{
    Tag {
        tag_name: String,
        attributes: Vec<(String, String)>,
        children: Vec<HtmlElem>
    },
    Text (String),
}

#[derive(Debug)]
enum TagNameType
{
    StartTag, EndTag, SelfClosingTag
}



pub fn parse_html(input: &str) -> Result<(), HtmlParseError>{
    let elements = Grammar::parse(Rule::html, input)
        .map_err(|_| HtmlParseError::ErrorHtmlStructure)?;
    
    
    for pair in elements {
        match pair.as_rule() {
            Rule::html => {
                for tag in pair.into_inner() {
                    if let Ok(elem) = parse_elem(&tag) {
                    }
                }
        
               }
      
               _ => ()
           }
       }
    Ok(())
}

pub fn parse_elem(pair: &Pair<Rule>) -> Result<HtmlElem, HtmlParseError> {
    // let tag = pair.into_inner().next().unwrap();
    println!{"{}", pair};

    match pair.as_rule() {
        Rule::elem => {
            let mut inner_pairs = pair.clone().into_inner();
            let start_tag = inner_pairs.next().unwrap();
            let tag_name = start_tag.into_inner().next().unwrap().as_str().to_string();
            println!("{}", "inner_pairs:");
            println!("{}", tag_name);
            
            for child in inner_pairs {
                match child.as_rule() {
                    Rule::elem => {
                        println!("{}", "CHILD TAG");
                        parse_elem(&child);
                    }
                    Rule::end_tag => {
                        println!("{}", "END TAG");

                    }
                    Rule:: text => {
                        println!("{}", "TEXT");

                    }
                    _ => return Err(HtmlParseError::ErrorHtmlStructure),
                }
            }

            // parse_elem(pair);
            // println!{"{}", inner_pairs};
            Ok(HtmlElem::Tag {
                tag_name: String::from("sk"),
                attributes: vec![],
                children: vec![],
            })
        }
        Rule::self_closed_tag => {
            let tag_name = pair.as_str().to_string();
            Ok(HtmlElem::Tag {
                tag_name,
                attributes: vec![],
                children: vec![],
            })
        }
        Rule::text => Ok(HtmlElem::Text(pair.as_str().to_string())),
        _ => Err(HtmlParseError::Unknown),
    }
    
    // Ok(HtmlElem::Text(String::from("kdjf")))
}
