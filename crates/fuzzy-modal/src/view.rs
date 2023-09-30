use common::{nvim, Rectangle};
use nvim::api::{
    types::{GotMode, Mode},
    Window,
};

use crate::*;

/// TODO: docs
pub(crate) struct Modal {
    /// TODO: docs
    prompt: Prompt,

    /// TODO: docs
    results: Results,

    /// TODO: docs
    layout: Box<dyn Layout>,

    /// TODO: docs
    ctx: Option<OpenCtx>,

    /// TODO: docs
    on_confirm: Option<OnConfirm>,

    /// TODO: docs
    on_cancel: Option<OnExit>,
}

/// TODO: docs
pub(crate) enum ConfirmResult {
    /// TODO: docs
    Confirmed,

    /// TODO: docs
    Ignored,
}

impl Modal {
    pub fn add_results(&mut self, new_results: Vec<FuzzyItem>) {
        self.results.extend(new_results);
        let total = self.results.num_total();
        self.prompt.update_total(total);
    }

    pub fn close(&mut self) {
        // self.layout.close().unwrap();

        self.prompt.close();

        if let Some(ctx) = self.ctx.take() {
            ctx.close();
        }

        let maybe_selected = self.results.close();

        if let Some(on_cancel) = self.on_cancel.take() {
            on_cancel(maybe_selected);
        }
    }

    pub fn closed(&mut self) {
        self.close();
    }

    /// TODO: docs
    pub fn confirm(&mut self) -> ConfirmResult {
        if let Some(selected_result) = self.results.take_selected() {
            if let Some(on_confirm) = self.on_confirm.take() {
                on_confirm(selected_result);
            }

            // After successfully confirming the view will be closed. If we
            // don't clear this callback it will be called by `Self::close`.
            self.on_cancel = None;

            ConfirmResult::Confirmed
        } else {
            ConfirmResult::Ignored
        }
    }

    pub fn id(&self) -> Option<ModalId> {
        self.ctx.as_ref().map(|ctx| ctx.id)
    }

    pub fn new(layout: Box<dyn Layout>, sender: Sender) -> Self {
        Self {
            prompt: Prompt::new(sender.clone()),
            results: Results::new(sender),
            layout,
            ctx: None,
            on_confirm: None,
            on_cancel: None,
        }
    }

    pub fn open(
        &mut self,
        FuzzyConfig { prompt, results, on_confirm, on_cancel }: FuzzyConfig,
        rectangle: Rectangle,
        modal_id: ModalId,
    ) {
        self.close();

        // TODO: switch
        self.results.open(results, modal_id);

        self.prompt.open(prompt, modal_id);

        self.layout
            .open(self.prompt.buffer(), self.results.buffer(), rectangle)
            .unwrap();

        self.ctx = Some(OpenCtx::open(modal_id));

        self.on_confirm = on_confirm;

        self.on_cancel = on_cancel;

        let _ = nvim::api::command("startinsert");
    }

    pub fn prompt(&self) -> &Prompt {
        &self.prompt
    }

    pub fn prompt_mut(&mut self) -> &mut Prompt {
        &mut self.prompt
    }

    pub fn results_mut(&mut self) -> &mut Results {
        &mut self.results
    }
}

/// TODO: docs
struct OpenCtx {
    /// TODO: docs
    id: ModalId,

    /// TODO: docs
    parent_window: Window,

    /// TODO: docs
    opened_in_mode: Mode,

    /// TODO: docs
    opened_at_position: Option<(usize, usize)>,
}

impl OpenCtx {
    /// TODO: docs
    fn close(mut self) {
        if self.opened_in_mode.is_insert() {
            return;
        }

        let _ = nvim::api::command("stopinsert");

        if let Some((line, col)) = self.opened_at_position {
            // I'm not really sure why it's necessary to add 1 to the original
            // column for the cursor to be placed at its original position if
            // the modal was opened while in normal mode.
            let _ = self.parent_window.set_cursor(line, col + 1);
        }
    }

    /// TODO: docs
    fn open(id: ModalId) -> Self {
        let current_mode =
            if let Ok(GotMode { mode, .. }) = nvim::api::get_mode() {
                mode
            } else {
                Mode::Normal
            };

        let parent_window = nvim::api::Window::current();

        let current_pos = parent_window.get_cursor().ok();

        Self {
            id,
            parent_window,
            opened_in_mode: current_mode,
            opened_at_position: current_pos,
        }
    }
}
