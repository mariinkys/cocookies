use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PdfTheme {
    #[default]
    Simple,
    Fresh,
    Dark,
    Pink,
}

impl Display for PdfTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let theme_name = match self {
            PdfTheme::Simple => "Simple",
            PdfTheme::Fresh => "Fresh",
            PdfTheme::Dark => "Dark",
            PdfTheme::Pink => "Pink",
        };
        write!(f, "{theme_name}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsePdfThemeError {
    pub invalid_value: String,
}

impl Display for ParsePdfThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid PDF theme: '{}'", self.invalid_value)
    }
}

impl std::error::Error for ParsePdfThemeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl FromStr for PdfTheme {
    type Err = ParsePdfThemeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Simple" => Ok(PdfTheme::Simple),
            "Fresh" => Ok(PdfTheme::Fresh),
            "Dark" => Ok(PdfTheme::Dark),
            "Pink" => Ok(PdfTheme::Pink),
            _ => Err(ParsePdfThemeError {
                invalid_value: s.to_string(),
            }),
        }
    }
}

impl PdfTheme {
    pub const ALL: &'static [Self] = &[Self::Simple, Self::Fresh, Self::Dark, Self::Pink];

    pub fn get_style(&self) -> &'static str {
        match &self {
            PdfTheme::Simple => simple_theme(),
            PdfTheme::Fresh => fresh_theme(),
            PdfTheme::Dark => dark_theme(),
            PdfTheme::Pink => pink_theme(),
        }
    }
}

fn simple_theme() -> &'static str {
    r#"
        <style>
            body {
                font-family: Arial, sans-serif;
                margin: 40px;
                line-height: 1.6;
                color: #333;
                background-color: #fff;
            }
            h1 {
                color: #2c3e50;
                border-bottom: 2px solid #ecf0f1;
                padding-bottom: 5px;
                margin-top: 40px;
            }
            p {
                margin: 10px 0;
            }
            .recipe-meta {
                font-style: italic;
                color: #7f8c8d;
            }
            .ingredient {
                padding: 6px;
                background: #f9f9f9;
                border-left: 4px solid #3498db;
                margin: 5px 0;
            }
            .step {
                background: #ecf0f1;
                padding: 10px;
                border-radius: 5px;
                margin: 10px 0;
            }
            .note {
                background: #fef9e7;
                padding: 10px;
                border-left: 4px solid #f1c40f;
                margin: 10px 0;
            }
            footer {
                text-align: center;
                margin-top: 30px;
                font-size: 14px;
                color: #7f8c8d;
                font-style: italic;
                border-top: 1px solid #ecf0f1;
                padding-top: 12px;
            }
            footer a {
                text-decoration: none;
                color: inherit;
            }
        </style>
    "#
}

fn fresh_theme() -> &'static str {
    r#"
        <style>
            body {
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                margin: 50px;
                line-height: 1.8;
                background-color: #fff;
                color: #2d3436;
            }
            h1 {
                font-size: 26px;
                color: #00cec9;
                border-bottom: 3px solid #dfe6e9;
                padding-bottom: 5px;
                margin-top: 40px;
            }
            p {
                margin: 10px 0;
            }
            .recipe-meta {
                font-size: 15px;
                color: #636e72;
            }
            .ingredient {
                background: #dff9fb;
                border-left: 6px solid #00b894;
                padding: 10px;
                margin: 8px 0;
                font-weight: 500;
            }
            .step {
                background: #ffeaa7;
                padding: 12px;
                border-radius: 6px;
                margin: 10px 0;
                border: 1px dashed #fdcb6e;
            }
            .note {
                background: #fab1a0;
                padding: 10px;
                margin: 10px 0;
                border-left: 5px solid #e17055;
                color: #2d3436;
            }
            footer {
                text-align: center;
                margin-top: 30px;
                font-size: 14px;
                color: #00b894;
                font-style: italic;
                border-top: 1px dashed #dfe6e9;
                padding-top: 12px;
            }
            footer a {
                text-decoration: none;
                color: inherit;
            }
        </style>
    "#
}

fn dark_theme() -> &'static str {
    r#"
        <style>
             body {
                font-family: 'Roboto', sans-serif;
                background-color: #1e1e1e;
                color: #f1f1f1;
                margin: 40px;
                line-height: 1.6;
            }
            h1 {
                color: #f39c12;
                border-bottom: 2px solid #444;
                padding-bottom: 5px;
                margin-top: 40px;
            }
            p {
                margin: 10px 0;
            }
            .recipe-meta {
                font-size: 14px;
                color: #aaa;
                font-style: italic;
            }
            .ingredient {
                background: #2c2c2c;
                padding: 10px;
                border-left: 4px solid #2980b9;
                margin: 6px 0;
                font-size: 15px;
            }
            .step {
                background: #3b3b3b;
                padding: 12px;
                border-radius: 5px;
                margin: 12px 0;
                border-left: 4px solid #9b59b6;
            }
            .note {
                background: #444;
                padding: 10px;
                border-left: 4px solid #f1c40f;
                margin: 10px 0;
                font-size: 14px;
            }
            footer {
                text-align: center;
                margin-top: 30px;
                font-size: 14px;
                color: #f39c12;
                font-style: italic;
                border-top: 1px dashed #444;
                padding-top: 12px;
            }
            footer a {
                text-decoration: none;
                color: inherit;
            }
        </style>
    "#
}

fn pink_theme() -> &'static str {
    r#"
        <style>
            body {
                font-family: 'Georgia', serif;
                background-color: #fff0f5;
                color: #4b2e2e;
                margin: 40px;
                line-height: 1.7;
            }
            h1 {
                font-size: 26px;
                color: #d63384;
                border-bottom: 2px dashed #f8bbd0;
                padding-bottom: 6px;
                margin-top: 40px;
            }
            p {
                margin: 10px 0;
            }
            .recipe-meta {
                font-size: 15px;
                color: #a05265;
                font-style: italic;
                margin-top: -10px;
            }
            .ingredient {
                padding: 10px 14px;
                background-color: #ffe4ec;
                border-left: 5px solid #e75480;
                margin: 8px 0;
                border-radius: 6px;
                font-size: 15px;
            }
            .step {
                background-color: #fce4ec;
                padding: 12px;
                border: 1px solid #f8bbd0;
                border-radius: 6px;
                margin: 12px 0;
                font-size: 15px;
            }
            .note {
                background-color: #f9d5e5;
                padding: 10px;
                border-left: 5px solid #d63384;
                margin: 10px 0;
                font-style: italic;
                font-size: 14px;
                color: #5a2c2c;
            }
            footer {
                text-align: center;
                margin-top: 30px;
                font-size: 14px;
                color: #d63384;
                font-style: italic;
                border-top: 1px dashed #f8bbd0;
                padding-top: 12px;
            }
            footer a {
                text-decoration: none;
                color: inherit;
            }
        </style>
    "#
}
