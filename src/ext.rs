use crate::mediawiki::{Node, TemplateArgument};

pub trait NodeExt {
    type Static;

    fn traverse(&self, f: &mut impl FnMut(&Node));
    fn to_string(&self, template: impl FnMut(&str, &[TemplateArgument], &mut String)) -> String;
    fn to_static(&self) -> Self::Static;
}

impl<'a> NodeExt for Node<'a> {
    type Static = Node<'static>;

    fn traverse(&self, f: &mut impl FnMut(&Node)) {
        f(self);

        if let Node::Template { arguments, .. } = self {
            for arg in arguments {
                arg.value.traverse(f);
            }
        }
    }

    fn to_string(
        &self,
        mut template: impl FnMut(&str, &[TemplateArgument], &mut String),
    ) -> String {
        let mut s = String::new();
        element_to_string(self, &mut s, &mut template);
        s
    }

    fn to_static(&self) -> Self::Static {
        match self {
            Node::Text(text) => Node::Text(text.clone().into_owned().into()),
            Node::Template { name, arguments } => Node::Template {
                name: name.clone().into_owned().into(),
                arguments: arguments
                    .iter()
                    .map(|arg| TemplateArgument {
                        name: arg
                            .name
                            .as_ref()
                            .map(|name| name.clone().into_owned().into()),
                        value: arg.value.to_static(),
                    })
                    .collect(),
            },
            Node::Error => Node::Error,
        }
    }
}

impl<'a> NodeExt for [Node<'a>] {
    type Static = Vec<Node<'static>>;

    fn traverse(&self, f: &mut impl FnMut(&Node)) {
        for elem in self {
            elem.traverse(f);
        }
    }

    fn to_string(
        &self,
        mut template: impl FnMut(&str, &[TemplateArgument], &mut String),
    ) -> String {
        let mut s = String::new();
        for elem in self {
            element_to_string(elem, &mut s, &mut template);
        }
        s
    }

    fn to_static(&self) -> Self::Static {
        self.iter().map(|elem| elem.to_static()).collect()
    }
}

fn element_to_string(
    elem: &Node,
    s: &mut String,
    mut template: impl FnMut(&str, &[TemplateArgument], &mut String),
) {
    match elem {
        Node::Text(text) => s.push_str(text),
        Node::Template { name, arguments } => {
            template(name, arguments, s);
        }
        Node::Error => {}
    }
}
