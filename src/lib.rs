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
    #[error("The tag is closed with a different tag than expected. Expected: {0}, found: {1}")]
    MismatchedClosingTag(String, String),
}
#[derive(Debug)]

pub enum HtmlElem {
    Tag {
        tag_name: String,
        children: Vec<HtmlElem>,
    },
    Text(String),
    Documentation(String),
}

pub fn parse_html(input: &str) -> Result<Vec<HtmlElem>, HtmlParseError> {
    let elements =
        Grammar::parse(Rule::document, input).map_err(|_| HtmlParseError::ErrorHtmlStructure)?;

    let mut html_dom = vec![];

    for pair in elements {
        match pair.as_rule() {
            Rule::document => {
                for tag in pair.into_inner() {
                    match tag.as_rule() {
                        Rule::elem | Rule::self_closed_tag => {
                            let elem = parse_elem(&tag)?;
                            println!("{:?}", elem);
                            html_dom.push(elem);
                        }
                        Rule::declaration => {
                            let doctype = tag.as_str().to_string();
                            html_dom.push(HtmlElem::Documentation(doctype));
                        }
                        _ => {}
                    }
                }
            }
            _ => return Err(HtmlParseError::ErrorHtmlStructure),
        }
    }
    Ok(html_dom)
}

pub fn parse_elem(pair: &Pair<Rule>) -> Result<HtmlElem, HtmlParseError> {
    match pair.as_rule() {
        Rule::elem => {
            let mut inner_pairs = pair.clone().into_inner();
            let start_tag = inner_pairs.next().unwrap();
            let tag_name = start_tag.into_inner().next().unwrap().as_str().to_string();

            let mut children = vec![];

            for child in inner_pairs {
                match child.as_rule() {
                    Rule::elem => {
                        let child_elem = parse_elem(&child)?;
                        children.push(child_elem);
                    }

                    Rule::text => {
                        let child_text = HtmlElem::Text(child.as_str().to_string());
                        children.push(child_text);
                    }

                    Rule::self_closed_tag => {
                        let child_elem = parse_elem(&child)?;
                        children.push(child_elem);
                    }

                    Rule::end_tag => {
                        let end_tag_name = child.into_inner().next().unwrap().as_str();
                        if tag_name != end_tag_name {
                            return Err(HtmlParseError::MismatchedClosingTag(
                                tag_name,
                                end_tag_name.to_string(),
                            ));
                        }
                    }
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
                children: vec![],
            })
        }

        _ => Err(HtmlParseError::ErrorHtmlStructure),
    }
}
