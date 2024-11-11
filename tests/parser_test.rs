use anyhow::{anyhow, Result};
use html_simple_parser::{parse_html, HtmlElem, HtmlParseError};

#[test]
fn thow_mismatched_closing_tag_error() -> Result<()> {
    let input = "<!DOCTYPE html>
                        <html>
                            <head> </head>
                        </div>";
    let result = parse_html(input);
    match result {
        Ok(_) => Err(anyhow!(
            "Test failed! There should be an error here due to tag mismatch."
        )),
        Err(HtmlParseError::MismatchedClosingTag(expected, found)) => {
            assert_eq!(expected, "html");
            assert_eq!(found, "div");
            Ok(())
        }
        Err(err) => Err(anyhow!(err).context("Test failed! Not the error that was expected.")),
    }
}

#[test]
fn invalid_html_structure() -> Result<()> {
    let input = "<!DOCTYPE html>
    <head> <head> <html> </head>
    </div>";
    let result = parse_html(input);
    match result {
        Ok(_) => Err(anyhow!(
            "Test failed! There should be an error here due to invalid structure."
        )),
        Err(HtmlParseError::ErrorHtmlStructure) => Ok(()),
        Err(err) => Err(anyhow!(err).context("Test failed! Not the error that was expected.")),
    }
}

#[test]
fn defining_right_tags() -> Result<()> {
    let input = "
    <!DOCTYPE html>
    <html> 
        <head>
            <p>Text</p>
            <br/>
        </head> 
    </html>";
    let result = parse_html(input);
    match result {
        Ok(elements) => {
            assert_eq!(elements.len(), 2);
            match &elements[0] {
                HtmlElem::Documentation(doctype) => {
                    assert_eq!("<!DOCTYPE html>", doctype);
                }
                _ => {
                    return Err(anyhow!(
                        "Test failed! First Element of html dom must be documentatation tag."
                    ))
                }
            }
            match &elements[1] {
                HtmlElem::Tag { tag_name, children } => {
                    assert_eq!("html", tag_name);
                    assert_eq!(children.len(), 1);
                    match &children[0] {
                        HtmlElem::Tag { tag_name, children } => {
                            assert_eq!("head", tag_name);
                            assert_eq!(children.len(), 2);
                            match &children[0] {
                                HtmlElem::Tag { tag_name, children } => {
                                    assert_eq!(tag_name, "p");
                                    assert_eq!(children.len(), 1);
                                    if let HtmlElem::Text(text) = &children[0] {
                                        assert_eq!(text, "Text");
                                    } else {
                                        return Err(anyhow!(
                                            "Test failed! <p> should contain text."
                                        ));
                                    }
                                }
                                _ => return Err(anyhow!(
                                    "Test failed! The first child of <head> should be a <p> tag."
                                )),
                            }
                            match &children[1] {
                                HtmlElem::Tag { tag_name, children } => {
                                    assert_eq!(tag_name, "<br/>");
                                    assert!(children.is_empty());
                                }
                                _ => return Err(anyhow!(
                                    "Test failed! The second child of <head> should be a <br> tag."
                                )),
                            }
                        }
                        _ => {
                            return Err(anyhow!(
                                "Test failed! The child of <html> should be a <head> tag."
                            ))
                        }
                    }
                }
                _ => return Err(anyhow!("Test failed! Second element must be tag.")),
            }
            Ok(())
        }
        Err(_) => Err(anyhow!("Test failed. HTML parsing failed.")),
    }
}
