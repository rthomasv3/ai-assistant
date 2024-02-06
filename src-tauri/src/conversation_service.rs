use llm::{models::Llama, ModelParameters};
// use llm::{Model, ModelParameters}; // main
use std::sync::Mutex;
use std::path::PathBuf;
use crate::models::message::Message;
use crate::models::conversation::Conversation;
use crate::Config;
use std::path::Path;

pub struct ConversationService {
    // pub model: Mutex<Box< dyn Model>>, // gguf
    pub model: Mutex<Option<Llama>>,
    pub conversation: Mutex<Conversation>,
    pub persona: &'static str,
    pub assistant_name: &'static str,
    pub user_name: &'static str,
}

impl ConversationService {
    pub fn new(config: &Config) -> Self {
        let model_params = ModelParameters {
            prefer_mmap: config.prefer_mmap,
            context_size: config.context_size,
            use_gpu: config.use_gpu,
            gpu_layers: config.gpu_layers,
            ..Default::default()
        };

        // gguf way to load model
        // let model = llm::load(
        //     &PathBuf::from(&model_path),
        //     llm::TokenizerSource::Embedded,
        //     model_params,
        //     llm::load_progress_callback_stdout
        // )
        // .unwrap_or_else(|err| panic!("Failed to load model from {model_path:?}: {err}"));

        let mut model: Option<Llama> = None;
        
        if config.model_path.is_some() {
            let model_path = config.model_path.clone().unwrap();

            if Path::new(&model_path).exists() {
                model = Some(llm::load::<Llama>(
                    &PathBuf::from(&model_path),
                    llm::TokenizerSource::Embedded,
                    model_params,
                    llm::load_progress_callback_stdout
                )
                .unwrap_or_else(|err| panic!("Failed to load mode from {model_path:?}: {err}")));
            }
        }

        let conversation = Self::get_default_conversation();

        Self {
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

    pub fn update_model(&self, config: &Config) -> bool {
        let mut loaded_model = false;
        
        // unload old model
        *self.model.lock().unwrap() = None;

        // load new model
        if config.model_path.is_some() {
            let model_path = config.model_path.clone().unwrap();

            if Path::new(&model_path).exists() {
                let model_params = ModelParameters {
                    prefer_mmap: config.prefer_mmap,
                    context_size: config.context_size,
                    use_gpu: config.use_gpu,
                    gpu_layers: config.gpu_layers,
                    ..Default::default()
                };

                let model = Some(llm::load::<Llama>(
                    &PathBuf::from(&model_path),
                    llm::TokenizerSource::Embedded,
                    model_params,
                    llm::load_progress_callback_stdout
                )
                .unwrap_or_else(|err| panic!("Failed to load mode from {model_path:?}: {err}")));
                *self.model.lock().unwrap() = model;
                loaded_model = true;
            }
        }

        return  loaded_model;
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
