use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Error, Debug)]
pub enum HtmlParseError {
    #[error("Invalid HTML structure detected!")]
    ErrorHtmlStructure,
    #[error("The tag is closed with a different tag than expected.")]
    MismatchedClosingTag,
    #[error("Unexpected html element detected!: {0}")]
    UnexpectedHtmlElement(String),
    #[error("unknown data store error")]
    Unknown,
}
#[derive(Debug)]
pub enum HtmlElem {
    Tag {
        tag_name: String,
        // attributes: Vec<(String, String)>,
        children: Vec<HtmlElem>,
    },
    Text(String),
    Documentation(String),
}

#[derive(Debug)]
enum TagNameType {
    StartTag,
    EndTag,
    SelfClosingTag,
}

pub fn parse_html(input: &str) -> Result<Vec<HtmlElem>, HtmlParseError> {
    let elements =
        Grammar::parse(Rule::html, input).map_err(|_| HtmlParseError::ErrorHtmlStructure)?;

    let mut htmlDom = vec![];
    for pair in elements {
        match pair.as_rule() {
            Rule::html => {
                for tag in pair.into_inner() {
                    match tag.as_rule() {
                        Rule::elem | Rule::self_closed_tag => {
                            if let Ok(elem) = parse_elem(&tag) {
                                println!("{}", "res!!!!!!!!!!!:");
                                println!("{:?}", elem);
                                htmlDom.push(elem);
                            }
                        }
                        Rule::declaration => {
                            let doctype = tag.as_str().to_string();
                            htmlDom.push(HtmlElem::Documentation(doctype));
                        }
                        _ => {}
                    }
                }
            }
            _ => return Err(HtmlParseError::ErrorHtmlStructure),
        }
    }
    println!("{:?}", htmlDom);
    Ok(htmlDom)
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
                        if let Ok(child_elem) = parse_elem(&child) {
                            children.push(child_elem);
                        }
                    }
                    Rule::text => {
                        let child_text = HtmlElem::Text(child.as_str().to_string());
                        children.push(child_text);
                        println!("{}", "TEXT");
                    }
                    Rule::self_closed_tag => {
                        if let Ok(child_elem) = parse_elem(&child) {
                            children.push(child_elem);
                        }
                    }
                    Rule::end_tag => {
                        let end_tag_name = child.into_inner().next().unwrap().as_str();
                        println!("{} {}'", tag_name, end_tag_name);

                        if tag_name != end_tag_name {
                            println!(
                                "Error: Mismatched tags detected! Expected '{}', found '{}'",
                                tag_name, end_tag_name
                            );
                            return Err(HtmlParseError::MismatchedClosingTag);
                        }
                        println!("End tag matched: {}", end_tag_name);

                        println!("{}", "END_TAG:");
                    }
                    // _ => {}
                    _ => return Err(HtmlParseError::ErrorHtmlStructure),
                }
            }
            Ok(HtmlElem::Tag { tag_name, children })
        }
        Rule::text => Ok(HtmlElem::Text(pair.as_str().to_string())),
        Rule::self_closed_tag => {
            let tag_name = pair.as_str().to_string();
            Ok(HtmlElem::Tag {
                tag_name,
                // attributes: vec![],
                children: vec![],
            })
        }

        _ => {
            println!("{}", pair);
            Err(HtmlParseError::Unknown)
        }
    }
}
