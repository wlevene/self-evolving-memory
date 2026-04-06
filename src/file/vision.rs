use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

/// GLM-4V vision API request
#[derive(Serialize)]
struct VisionRequest {
    model: String,
    messages: Vec<VisionMessage>,
}

#[derive(Serialize)]
struct VisionMessage {
    role: String,
    content: Vec<VisionContent>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum VisionContent {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize)]
struct ImageUrl {
    url: String,
}

#[derive(Deserialize)]
struct VisionResponse {
    choices: Vec<VisionChoice>,
}

#[derive(Deserialize)]
struct VisionChoice {
    message: VisionResponseMessage,
}

#[derive(Deserialize)]
struct VisionResponseMessage {
    content: String,
}

/// Generate description for image using GLM-4V
pub async fn describe_image(image_data: &[u8], api_key: Option<String>) -> Result<String, String> {
    let base64_image = general_purpose::STANDARD.encode(image_data);
    let image_url = format!("data:image/jpeg;base64,{}", base64_image);
    
    // Use GLM-4V via dashscope or OpenAI-compatible endpoint
    let endpoint = std::env::var("GLM_API_ENDPOINT")
        .unwrap_or("https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions".to_string());
    
    // Get API key from parameter or environment
    let api_key = api_key
        .or_else(|| std::env::var("GLM_API_KEY").ok())
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
        .ok_or_else(|| "No API key configured for vision model".to_string())?;
    
    let request = VisionRequest {
        model: "glm-4v".to_string(),
        messages: vec![
            VisionMessage {
                role: "user".to_string(),
                content: vec![
                    VisionContent::Text { 
                        text: "请详细描述这张图片的内容，包括：主要元素、场景、色调、情感氛围。用于记忆系统存储。" .to_string()
                    },
                    VisionContent::ImageUrl { 
                        image_url: ImageUrl { url: image_url }
                    },
                ],
            },
        ],
    };
    
    let client = reqwest::Client::new();
    let response = client
        .post(&endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Vision API request failed: {}", e))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Vision API error {}: {}", status, body));
    }
    
    let vision_response: VisionResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse vision response: {}", e))?;
    
    vision_response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| "No content in vision response".to_string())
}

/// Supported image formats
pub fn is_image_format(content_type: &str) -> bool {
    content_type.starts_with("image/")
}

/// Get image format from content type
pub fn get_image_format(content_type: &str) -> String {
    content_type
        .split('/')
        .nth(1)
        .unwrap_or("unknown")
        .to_string()
}