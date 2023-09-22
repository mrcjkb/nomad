use crate::*;

pub(crate) struct View {
    prompt: Prompt,
    results: Results,
}

impl View {
    pub fn new(config: FuzzyConfig) -> Self {
        let FuzzyConfig {
            items,
            on_confirm,
            on_cancel,
            on_select,
            starting_text,
            starting_selected,
        } = config;

        // let len = self.items.len();
        //
        // let prompt = Prompt::new(
        //     self.starting_text.clone(),
        //     self.items.len() as _,
        //     move |query| {
        //         nvim::print!("new query is {query}");
        //         len as _
        //     },
        // );
        //
        // self.prompt = Some(prompt);

        todo!();
    }

    pub fn close(self) {
        self.prompt.close();
        self.results.close();
    }
}
