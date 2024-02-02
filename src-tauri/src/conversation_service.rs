use llm::{models::Llama, ModelParameters};
// use llm::{Model, ModelParameters}; // main
use std::sync::Mutex;
use std::path::PathBuf;
use crate::{models::message::Message, Conversation};

pub struct ConversationService {
    // pub model: Mutex<Box< dyn Model>>, // gguf
    pub model: Mutex<Llama>,
    pub conversation: Mutex<Conversation>,
    pub persona: &'static str,
    pub assistant_name: &'static str,
    pub user_name: &'static str,
}

impl ConversationService {
    pub fn new() -> ConversationService {
        dotenv::dotenv().ok();
        let model_path = std::env::var("MODEL_PATH").expect("MODEL_PATH must be set");

        let model_params = ModelParameters {
            prefer_mmap: true,
            use_gpu: true,
            context_size: 1024,
            ..Default::default()
        };

        // gguf
        // let model = llm::load(
        //     &PathBuf::from(&model_path),
        //     llm::TokenizerSource::Embedded,
        //     model_params,
        //     llm::load_progress_callback_stdout
        // )
        // .unwrap_or_else(|err| panic!("Failed to load model from {model_path:?}: {err}"));

        let model = llm::load::<Llama>(
            &PathBuf::from(&model_path),
            llm::TokenizerSource::Embedded,
            model_params,
            llm::load_progress_callback_stdout
        )
        .unwrap_or_else(|err| panic!("Failed to load mode from {model_path:?}: {err}"));

        let conversation = Self::get_default_conversation();

        ConversationService {
            model: Mutex::from(model),
            conversation: Mutex::from(conversation),
            persona: "Transcript of a dialog, where the User interacts with an Assistant named Bob. Bob is helpful, kind, honest, good at writing, and never fails to answer the User's requests immediately and with precision.",
            assistant_name: "Bob:",
            user_name: "User:"
        }
    }
    
    pub fn add_message(&self, is_user: bool, text: &str) {
        self.conversation.lock().unwrap().messages.push(Message {
            is_user: is_user,
            text: text.to_string()
        });
    }

    pub fn reset_conversation(&self) {
        *self.conversation.lock().unwrap() = Self::get_default_conversation();
    }

    fn get_default_conversation() -> Conversation {
        let mut messages: Vec<Message> = Vec::new();

        messages.push(Message{
            is_user: true,
            text: String::from("Hello, Bob.")
        });
        messages.push(Message{
            is_user: false,
            text: String::from("Hello. How may I help you today?")
        });
        messages.push(Message{
            is_user: true,
            text: String::from("Please tell me the largest city in Europe.")
        });
        messages.push(Message{
            is_user: false,
            text: String::from("The largest city in Europe is Moscow, the capital of Russia.")
        });

        Conversation {
            messages: messages
        }
    }
}
