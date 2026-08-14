pub(crate) struct View {
    nodes: Vec<Node>,
}

impl View {
    pub(crate) fn new() -> View {
        View { nodes: Vec::new() }
    }

    pub(crate) fn clear(&mut self) -> UiRoot<'_> {
        self.nodes.clear();

        UiRoot { view: self }
    }
}

struct Node {
    // TODO: Use an interned string type.
    #[expect(unused, reason = "TODO")]
    kind: Kind<String>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Kind<String> {
    Text { content: String },
}

pub struct UiRoot<'view> {
    view: &'view mut View,
}

impl UiRoot<'_> {
    pub fn plain_text(self, text: &str) {
        debug_assert!(self.view.nodes.is_empty());

        self.view.nodes.push(Node {
            kind: Kind::Text {
                content: text.to_owned(),
            },
        });
    }
}
