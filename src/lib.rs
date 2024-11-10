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
        // attributes: Vec<(String, String)>,
        children: Vec<HtmlElem>
    },
    Text (String),
    Documentation (String),
}

#[derive(Debug)]
enum TagNameType
{
    StartTag, EndTag, SelfClosingTag
}



pub fn parse_html(input: &str) -> Result<(), HtmlParseError>{
    let elements = Grammar::parse(Rule::html, input)
        .map_err(|_| HtmlParseError::ErrorHtmlStructure)?;
    
    let mut htmlDom = vec![];
    for pair in elements {
        match pair.as_rule() {
            Rule::html => {
                for tag in pair.into_inner() {
                    if let Ok(elem) = parse_elem(&tag) {
                        println!("{}", "res!!!!!!!!!!!:");
                        println!("{:?}", elem);

                    }
                    else {
                        println!("{}", "ERROR:");

                    }

                }
            }
            Rule::declaration => {
                let name = pair.as_str().to_string();
                htmlDom.push(HtmlElem::Documentation(name));
            }
      
               _ => ()
           }
       }
    Ok(())
}

pub fn parse_elem(pair: &Pair<Rule>) -> Result<HtmlElem, HtmlParseError> {
    // let tag = pair.into_inner().next().unwrap();
    // println!{"{}", pair};
    
    match pair.as_rule() {
        Rule::elem => {
            let mut inner_pairs = pair.clone().into_inner();
            let start_tag = inner_pairs.next().unwrap();
            let tag_name = start_tag.into_inner().next().unwrap().as_str().to_string();

            println!("{}", "inner_pairs:");
            println!("{}", tag_name);
    
            let mut children = vec![];

            for child in inner_pairs {

                println!("{:?}", child);

                match child.as_rule() {
                    Rule::elem => {
                        println!("{}", "CHILD TAG");
                        if let Ok(child_elem) = parse_elem(&child){
                            children.push(child_elem);
                        }
                    }
                    Rule:: text => {
                        let child_text = HtmlElem::Text(child.as_str().to_string());
                        children.push(child_text);
                        println!("{}", "TEXT");

                    }
                    Rule::self_closed_tag => {
                        if let Ok(child_elem) = parse_elem(&child){
                            children.push(child_elem);
                        }

                    }
                    Rule::end_tag => {
                        println!("{}", "END_TAG:");

                    }
                    // _ => {}
                    _ => return Err(HtmlParseError::ErrorHtmlStructure),
                }
            }
            Ok(HtmlElem:: Tag { tag_name, children})
        }
        Rule::text => Ok(HtmlElem::Text(pair.as_str().to_string())),
        Rule::self_closed_tag  => {
            let tag_name = pair.as_str().to_string();
            Ok(HtmlElem::Tag {
                tag_name,
                // attributes: vec![],
                children: vec![],
            })
        }

        _ => {
            println!("{}", pair);
            Err(HtmlParseError::Unknown)},
    }
    
}
