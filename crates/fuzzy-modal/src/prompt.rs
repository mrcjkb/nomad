use common::{nvim, WindowConfig, *};
use nvim::api::{opts::*, types::*, Buffer, Window};

use crate::*;

#[derive(Default)]
pub(crate) struct PromptConfig {
    /// A placeholder text to display when the prompt is empty.
    pub placeholder_text: Option<String>,

    /// The size of the result space over which the prompt query is matched.
    /// This remains constant between [`Prompt::open`] calls and is displayed
    /// at the end of the prompt together with the current number of matched
    /// results.
    pub total_results: u64,
}

/// TODO: docs
pub(crate) struct Prompt {
    /// The current value of the prompt, which is used as the query to filter
    /// the results. Its "rest" value should always match the one of the
    /// [`Results::query`] field.
    value: String,

    /// The number of results that match the current prompt. This is updated
    /// as the user types and it's displayed at the end of the prompt together
    /// with the total number of results.
    matched_results: u64,

    /// A sender used to send [`Message::PromptChanged`] messages to the parent
    /// plugin when the prompt changes.
    sender: Sender<Message>,

    /// The current configuration of the prompt, which changes every time the
    /// prompt is opened.
    config: PromptConfig,

    /// The buffer used to display the prompt.
    buffer: Buffer,

    /// The window that houses the buffer. This is only set when the prompt is
    /// open.
    window: Option<Window>,

    /// TODO: docs.
    namespace_id: u32,

    /// TODO: docs.
    placeholder_extmark_id: Option<u32>,

    /// TODO: docs.
    matched_on_total_extmark_id: Option<u32>,
}

impl Prompt {
    /// TODO: docs
    pub fn close(&mut self) {
        if let Some(window) = self.window.take() {
            // This will fail if the window is already closed.
            let _ = window.close(true);
        }

        self.update_placeholder("");
    }

    /// TODO: docs
    pub fn open(
        &mut self,
        config: PromptConfig,
        window_config: &WindowConfig,
    ) {
        if let Some(placeholder) = config.placeholder_text.as_ref() {
            self.update_placeholder(placeholder);
        }

        self.update_matched_on_total(
            config.total_results,
            config.total_results,
        );

        let window =
            nvim::api::open_win(&self.buffer, true, &window_config.into())
                .unwrap();

        nvim::api::command("startinsert").unwrap();

        self.matched_results = config.total_results;

        self.window = Some(window);

        self.config = config;
    }

    /// Initializes the prompt.
    ///
    /// TODO: docs.
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            value: String::new(),
            matched_results: 0,
            sender,
            config: PromptConfig::default(),
            buffer: nvim::api::create_buf(false, true).unwrap(),
            window: None,
            // Create an anonymous namespace for the prompt.
            namespace_id: nvim::api::create_namespace(""),
            placeholder_extmark_id: None,
            matched_on_total_extmark_id: None,
        }
    }

    /// TODO: docs
    pub fn update_matched(&mut self, new_matched_results: u64) {
        self.matched_results = new_matched_results;

        self.update_matched_on_total(
            self.matched_results,
            self.config.total_results,
        );
    }

    /// TODO: docs
    fn update_matched_on_total(&mut self, new_matched: u64, new_total: u64) {
        if let Some(old_extmark) = self.matched_on_total_extmark_id {
            self.buffer.del_extmark(self.namespace_id, old_extmark).unwrap();
        }

        let new_matched_on_total =
            format_matched_on_total(new_matched, new_total);

        let new_extmark = self
            .buffer
            .set_extmark(
                self.namespace_id,
                0,
                0,
                &SetExtmarkOpts::builder()
                    .virt_text([(
                        new_matched_on_total,
                        highlights::PROMPT_MATCHED_ON_TOTAL,
                    )])
                    .virt_text_pos(ExtmarkVirtTextPosition::RightAlign)
                    .build(),
            )
            .unwrap();

        self.placeholder_extmark_id = Some(new_extmark);
    }

    /// TODO: docs
    fn update_placeholder(&mut self, new_placeholder: &str) {
        if let Some(old_extmark) = self.placeholder_extmark_id {
            self.buffer.del_extmark(self.namespace_id, old_extmark).unwrap();
        }

        let new_extmark = self
            .buffer
            .set_extmark(
                self.namespace_id,
                0,
                0,
                &SetExtmarkOpts::builder()
                    .virt_text([(
                        new_placeholder,
                        highlights::PROMPT_PLACEHOLDER,
                    )])
                    .virt_text_pos(ExtmarkVirtTextPosition::Overlay)
                    .build(),
            )
            .unwrap();

        self.placeholder_extmark_id = Some(new_extmark);
    }

    /// TODO: docs
    pub fn update_total(&mut self, new_total_results: u64) {
        self.config.total_results = new_total_results;

        self.update_matched_on_total(
            self.matched_results,
            self.config.total_results,
        );
    }
}

/// TODO: docs
fn format_matched_on_total(
    matched: u64,
    total: u64,
) -> impl Into<nvim::String> {
    let formatted = format!("{}/{}", matched, total);
    nvim::String::from(formatted.as_str())
}
