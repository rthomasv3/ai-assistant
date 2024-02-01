// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod conversation_service;

use crate::models::conversation::Conversation;
use conversation_service::ConversationService;
use llm::InferenceSessionConfig;
use llm::KnownModel; // main
use tauri::State;
use std::convert::Infallible;

#[tauri::command]
async fn send_message(text: &str, conversation_service: State<'_, ConversationService>) -> Result<String, ()> {
    // add the user message to conversation history
    conversation_service.add_message(true, text);
    // get response from model
    let model_response = get_model_response(conversation_service.inner());
    // add response to conversation
    conversation_service.add_message(false, &model_response);
    
    return Ok(model_response);
}

fn main() {
    let conversation_service = ConversationService::new();

    tauri::Builder::default()
        .manage(conversation_service)
        .invoke_handler(tauri::generate_handler![send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_model_response(conversation_service: &ConversationService) -> String {
    let assistant_name = conversation_service.assistant_name;
    let user_name = conversation_service.user_name;
    let persona = conversation_service.persona;
    let mut history = String::from("");

    for message in &conversation_service.conversation.lock().unwrap().messages {
        let message_text = &message.text;
        let current_line = if message.is_user {
            format!("{user_name} {message_text}\n")
        } else {
            format!("{assistant_name} {message_text}\n")
        };
        history.push_str(&current_line);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();

    // let model = &*conversation_service.model.lock().unwrap(); // gguf
    let model = conversation_service.model.lock().unwrap();

    let session_config = InferenceSessionConfig {
        n_batch: 256,
        ..Default::default()
    };

    let mut session = model.start_session(session_config);

    session.infer::<Infallible>(
        // model.as_ref(), // gguf
        &*model,
        &mut rng,
        &llm::InferenceRequest {
            prompt: format!("{persona}\n\n{history}\n{assistant_name}").as_str().into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: Some(1024) // None
        },
        &mut Default::default(),
        inference_callback(String::from(user_name), &mut buf, & mut res),
    ).unwrap_or_else(|e| panic!("{e}"));

    res
}

fn inference_callback<'a>(
    stop_sequence: String,
    buf: &'a mut String,
    out_str: &'a mut String,
) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
    use llm::InferenceFeedback::Halt;
    use llm::InferenceFeedback::Continue;

    move |resp| -> Result<llm::InferenceFeedback, Infallible> {match resp {
        llm::InferenceResponse::InferredToken(t) => {
            let mut reverse_buf = buf.clone();
            reverse_buf.push_str(t.as_str());
            if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                buf.clear();
                return Ok(Halt);
            } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                buf.push_str(t.as_str());
                return Ok(Continue);
            }

            // Clone the string we're going to send
            if buf.is_empty() {
                out_str.push_str(&t);
            } else {
                out_str.push_str(&reverse_buf);
            }

            Ok(Continue)
        }
        llm::InferenceResponse::EotToken => Ok(Halt),
        _ => Ok(Continue),
    }}
}
