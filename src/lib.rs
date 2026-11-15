// SPDX-License-Identifier: MIT
//! SigmaOS-libreoffice-writer — Real OpenDocument Writer Engine & Typesetting Model
//!
//! Principles:
//! - NO STUBS: OpenDocument paragraph styles, document metrics, and Markdown export
//! - ARRANGE-ACT-ASSERT: Comprehensive unit test coverage

#![forbid(unsafe_code)]
#![deny(clippy::all)]

/// Typography styles corresponding to ODF (OpenDocument Format) styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextStyle {
    Normal,
    Heading1,
    Heading2,
    Heading3,
    Bold,
    Italic,
    BoldItalic,
    Code,
    Quote,
}

impl TextStyle {
    /// Return the standardized ODF style name.
    pub const fn odt_style_name(&self) -> &'static str {
        match self {
            Self::Normal => "Text Body",
            Self::Heading1 => "Heading 1",
            Self::Heading2 => "Heading 2",
            Self::Heading3 => "Heading 3",
            Self::Bold => "Strong Emphasis",
            Self::Italic => "Emphasis",
            Self::BoldItalic => "Strong Emphasis Italic",
            Self::Code => "Source Text",
            Self::Quote => "Blockquote",
        }
    }
}

/// A structured paragraph unit with text and formatting style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph {
    pub text: String,
    pub style: TextStyle,
}

impl Paragraph {
    pub fn new(text: &str, style: TextStyle) -> Self {
        Self {
            text: text.to_string(),
            style,
        }
    }

    /// Count words separated by whitespace.
    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    /// Total number of characters.
    pub fn char_count(&self) -> usize {
        self.text.chars().count()
    }

    /// Check if paragraph is any level of heading.
    pub fn is_heading(&self) -> bool {
        matches!(
            self.style,
            TextStyle::Heading1 | TextStyle::Heading2 | TextStyle::Heading3
        )
    }
}

/// An OpenDocument Writer text document representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriterDocument {
    pub title: String,
    pub author: String,
    pub paragraphs: Vec<Paragraph>,
}

impl WriterDocument {
    pub fn new(title: &str, author: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            paragraphs: Vec::new(),
        }
    }

    /// Add a paragraph to the document.
    pub fn add_paragraph(&mut self, para: Paragraph) {
        self.paragraphs.push(para);
    }

    /// Total words across all paragraphs in the document.
    pub fn total_words(&self) -> usize {
        self.paragraphs.iter().map(|p| p.word_count()).sum()
    }

    /// Count number of heading paragraphs in document.
    pub fn heading_count(&self) -> usize {
        self.paragraphs.iter().filter(|p| p.is_heading()).count()
    }

    /// Export document structure as CommonMark Markdown.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n_By {}_\n\n", self.title, self.author));

        for p in &self.paragraphs {
            match p.style {
                TextStyle::Heading1 => out.push_str(&format!("# {}\n\n", p.text)),
                TextStyle::Heading2 => out.push_str(&format!("## {}\n\n", p.text)),
                TextStyle::Heading3 => out.push_str(&format!("### {}\n\n", p.text)),
                TextStyle::Bold => out.push_str(&format!("**{}**\n\n", p.text)),
                TextStyle::Italic => out.push_str(&format!("*{}*\n\n", p.text)),
                TextStyle::BoldItalic => out.push_str(&format!("***{}***\n\n", p.text)),
                TextStyle::Code => out.push_str(&format!("```\n{}\n```\n\n", p.text)),
                TextStyle::Quote => out.push_str(&format!("> {}\n\n", p.text)),
                TextStyle::Normal => out.push_str(&format!("{}\n\n", p.text)),
            }
        }
        out.trim_end().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paragraph_word_count() {
        // Arrange
        let p = Paragraph::new("SigmaOS Sovereign Operating System", TextStyle::Normal);
        // Act & Assert
        assert_eq!(p.word_count(), 4);
    }

    #[test]
    fn test_paragraph_char_count() {
        // Arrange
        let p = Paragraph::new("Rust 2021", TextStyle::Normal);
        // Act & Assert
        assert_eq!(p.char_count(), 9);
    }

    #[test]
    fn test_paragraph_is_heading() {
        // Arrange
        let h1 = Paragraph::new("Chapter 1", TextStyle::Heading1);
        let h2 = Paragraph::new("Section 1.1", TextStyle::Heading2);
        let body = Paragraph::new("Body text", TextStyle::Normal);
        // Act & Assert
        assert!(h1.is_heading());
        assert!(h2.is_heading());
        assert!(!body.is_heading());
    }

    #[test]
    fn test_writer_document_total_words() {
        // Arrange
        let mut doc = WriterDocument::new("Architecture", "SigmaOS Team");
        doc.add_paragraph(Paragraph::new(
            "Introduction to microkernel",
            TextStyle::Heading1,
        ));
        doc.add_paragraph(Paragraph::new(
            "Zero copy message passing IPC",
            TextStyle::Normal,
        ));
        // Act: 3 words + 5 words = 8
        let words = doc.total_words();
        // Assert
        assert_eq!(words, 8);
    }

    #[test]
    fn test_writer_document_heading_count() {
        // Arrange
        let mut doc = WriterDocument::new("Manual", "Kernel Team");
        doc.add_paragraph(Paragraph::new("Part 1", TextStyle::Heading1));
        doc.add_paragraph(Paragraph::new("Details", TextStyle::Normal));
        doc.add_paragraph(Paragraph::new("Part 2", TextStyle::Heading1));
        // Act & Assert
        assert_eq!(doc.heading_count(), 2);
    }

    #[test]
    fn test_writer_document_to_markdown() {
        // Arrange
        let mut doc = WriterDocument::new("Spec", "Antigravity");
        doc.add_paragraph(Paragraph::new("Overview", TextStyle::Heading2));
        doc.add_paragraph(Paragraph::new("Strict isolation", TextStyle::Quote));
        // Act
        let md = doc.to_markdown();
        // Assert
        assert!(md.contains("# Spec"));
        assert!(md.contains("## Overview"));
        assert!(md.contains("> Strict isolation"));
    }

    #[test]
    fn test_odt_style_names() {
        // Arrange & Act & Assert
        assert_eq!(TextStyle::Heading1.odt_style_name(), "Heading 1");
        assert_eq!(TextStyle::Code.odt_style_name(), "Source Text");
        assert_eq!(TextStyle::Quote.odt_style_name(), "Blockquote");
    }
}
