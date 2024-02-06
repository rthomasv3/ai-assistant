// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod conversation_service;

use crate::models::config::Config;
use conversation_service::ConversationService;
use llm::KnownModel; // main
use llm::InferenceSessionConfig;
use tauri::Manager;
use tauri::State;
use tauri::api::dialog;
use std::convert::Infallible;
use std::fs;
use std::sync::Mutex;

type WrappedConfig = Mutex<Option<Config>>;

#[tauri::command]
async fn send_message(text: &str, conversation_service: State<'_, ConversationService>) -> Result<String, String> {
    let result: Result<String, String>;

    if conversation_service.model.lock().unwrap().is_some() {
        // add the user message to conversation history
        conversation_service.add_message(true, text);
        // get response from model
        let model_response = get_model_response(conversation_service.inner());
        // add response to conversation
        conversation_service.add_message(false, &model_response);

        result = Ok(model_response);
    } else {
        result = Err("No model loaded.".into())
    }

    result
}

#[tauri::command]
fn reset_conversation(conversation_service: State<'_, ConversationService>) {
    conversation_service.reset_conversation();
}

#[tauri::command]
fn get_config(config: State<'_, WrappedConfig>) -> Result<Option<Config>, ()> {
    Ok(config.lock().unwrap().clone())
}

#[tauri::command]
fn get_models(config: State<'_, WrappedConfig>) -> Vec<String> {
    let mut models: Vec<String> = Vec::new();

    if let Some(ref config) = *config.lock().unwrap() {
        let model_folder = config.model_folder.clone().unwrap();
        let file_paths = fs::read_dir(model_folder).unwrap();

        for path in file_paths {
            let path_buf = path.unwrap().path();
            let extension = path_buf.extension();
            if extension.is_some() && extension.unwrap() == "bin" {
                models.push(String::from(path_buf.to_str().unwrap()));
            }
        }
    }

    models
}

#[tauri::command]
async fn update_model(model_path: &str, config: State<'_, WrappedConfig>, conversation_service: State<'_, ConversationService>) -> Result<(), String> {
    if let Some(ref mut config) = *config.lock().unwrap() {
        config.update_model_path(model_path);
        conversation_service.update_model(config);
    }
    Ok(())
}

#[tauri::command]
async fn update_config(new_config: Config, old_config: State<'_, WrappedConfig>, conversation_service: State<'_, ConversationService>) -> Result<bool, String> {
    let mut loaded_model = false;
    
    *old_config.lock().unwrap() = Some(new_config);
    if let Some(ref config) = *old_config.lock().unwrap() {
        loaded_model = conversation_service.update_model(config);
    }
    Ok(loaded_model)
}

#[tauri::command]
async fn initialize(app: tauri::AppHandle) -> Result<bool, ()> {
    let config = Config::new();
    let conversation_service = ConversationService::new(&config);
    let loaded_model = conversation_service.model.lock().unwrap().is_some();
    app.manage(conversation_service);
    app.manage(Mutex::new(Some(config)));
    Ok(loaded_model)
}

#[tauri::command]
async fn pick_folder(app: tauri::AppHandle) -> Result<(), ()> {
    dialog::FileDialogBuilder::new().pick_folder(move |folder_path| {
        if folder_path.is_some() {
            app.emit_all("folder_picked", folder_path.unwrap().to_str().unwrap()).unwrap();
        }
    });
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![initialize, send_message, reset_conversation, get_config, get_models, update_model, update_config, pick_folder])
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
    let model_guard = conversation_service.model.lock().unwrap();
    let model_option = model_guard.as_ref();
    let model = model_option.unwrap();

    let session_config = InferenceSessionConfig {
        n_batch: 256,
        ..Default::default()
    };

    let mut session = model.start_session(session_config);

    session.infer::<Infallible>(
        // model.as_ref(), // gguf
        model,
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
