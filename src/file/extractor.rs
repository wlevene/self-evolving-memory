use std::path::Path;
use std::fs;
use std::process::Command;

/// Extracted document content
#[derive(Debug, Clone)]
pub struct ExtractedContent {
    pub text: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: u64,
}

/// Extract text from a file
pub fn extract_file(path: &Path) -> Result<ExtractedContent, String> {
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }
    
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let file_size = fs::metadata(path)
        .map(|m| m.len())
        .unwrap_or(0);
    
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let text = match extension.as_str() {
        "txt" | "md" | "text" => extract_text_file(path)?,
        "pdf" => extract_pdf(path)?,
        "docx" | "doc" => extract_docx(path)?,
        "rtf" => extract_rtf(path)?,
        _ => return Err(format!("Unsupported file type: {}", extension)),
    };
    
    // Limit content size
    let text = if text.len() > 10000 {
        text[..10000].to_string() + "\n\n... [content truncated]"
    } else {
        text
    };
    
    Ok(ExtractedContent {
        text,
        file_name,
        file_type: extension,
        file_size,
    })
}

/// Extract plain text file
fn extract_text_file(path: &Path) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("Failed to read text file: {}", e))
}

/// Extract PDF using external tool
fn extract_pdf(path: &Path) -> Result<String, String> {
    // Try pdftotext (common on Linux/macOS)
    let output = Command::new("pdftotext")
        .arg("-layout")
        .arg(path)
        .arg("-")
        .output();
    
    match output {
        Ok(o) if o.status.success() => {
            Ok(String::from_utf8_lossy(&o.stdout).to_string())
        }
        _ => {
            // Fallback: try Python's pdfminer
            let output = Command::new("python3")
                .arg("-c")
                .arg("import sys; from pdfminer.high_level import extract_text; print(extract_text(sys.argv[1]))")
                .arg(path)
                .output();
            
            match output {
                Ok(o) if o.status.success() => {
                    Ok(String::from_utf8_lossy(&o.stdout).to_string())
                }
                _ => Err("PDF extraction requires pdftotext or pdfminer.six. Install: brew install poppler or pip install pdfminer.six".to_string()),
            }
        }
    }
}

/// Extract DOCX using external tool
fn extract_docx(path: &Path) -> Result<String, String> {
    // macOS: textutil
    let output = Command::new("textutil")
        .arg("-convert")
        .arg("txt")
        .arg("-stdout")
        .arg(path)
        .output();
    
    match output {
        Ok(o) if o.status.success() => {
            Ok(String::from_utf8_lossy(&o.stdout).to_string())
        }
        _ => {
            // Try pandoc
            let output = Command::new("pandoc")
                .arg("-f")
                .arg("docx")
                .arg("-t")
                .arg("plain")
                .arg(path)
                .output();
            
            match output {
                Ok(o) if o.status.success() => {
                    Ok(String::from_utf8_lossy(&o.stdout).to_string())
                }
                _ => Err("DOCX extraction requires textutil (macOS) or pandoc. Install: brew install pandoc".to_string()),
            }
        }
    }
}

/// Extract RTF using textutil (macOS)
fn extract_rtf(path: &Path) -> Result<String, String> {
    let output = Command::new("textutil")
        .arg("-convert")
        .arg("txt")
        .arg("-stdout")
        .arg(path)
        .output();
    
    match output {
        Ok(o) if o.status.success() => {
            Ok(String::from_utf8_lossy(&o.stdout).to_string())
        }
        _ => Err("RTF extraction requires textutil (macOS). No alternative available.".to_string()),
    }
}