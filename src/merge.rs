use crate::context::Context;
use find_matching_bracket::find_matching_curly_brace;
use indexmap::IndexMap;

pub struct ComponentMerger(IndexMap<String, Vec<String>>);

impl ComponentMerger {
    pub fn with_capacity(len: usize) -> Self {
        ComponentMerger(IndexMap::with_capacity(len))
    }
    pub fn parse(&self, context: &mut Context) {
        for (name, merges) in self.0.iter() {
            let mut target = context.swap_remove(name).unwrap();
            for merge in merges {
                let source = context.get(merge).unwrap().clone();
                target.extend(source);
            }
            context.insert(name.to_owned(), target);
        }
        for name in context.keys().cloned().collect::<Vec<String>>() {
            context.apply_template(&name);
        }
    }

    pub fn parse_name(&mut self, name: String) -> String {
        if let Some(start) = name.find("(") {
            let end = find_matching_curly_brace(&name, start)
                .expect(&format!("Missing closing merging bracket {:}", name));
            assert_eq!(end, name.len() - 1);
            let name = name[..start].to_owned();
            let merging_components = name[start + 1..end]
                .split(",")
                .map(|s| s.to_owned())
                .collect();
            self.0.insert(name.to_owned(), merging_components);
            name
        } else {
            name
        }
    }
}

impl Context {
    fn apply_template(&mut self, name: &str) {
        if let Some(target) = self.get(name).cloned() {
            if let Some(mut template) = self.swap_remove(&format!("${}", name)) {
                template.extend(target.clone());
                self.apply_template(name);
                self.insert(name.to_string(), template);
            }
        }
    }
}
