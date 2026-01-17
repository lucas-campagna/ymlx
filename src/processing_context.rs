use core::panic;
use std::ops::Deref;

use super::{Context, Value};
use boa_engine::{JsValue, Source, js_string, property::Attribute};
use find_matching_bracket::find_matching_curly_brace;

struct ProcessingContextCode<'a, 'b> {
    start: usize,
    end: usize,
    code: &'a str,
    result: Option<JsValue>,
    context: Option<&'b mut boa_engine::Context>,
}

impl<'a, 'b> ProcessingContextCode<'a, 'b> {
    fn build(start: usize, end: usize, text: &'a str) -> Self {
        ProcessingContextCode {
            start,
            end,
            code: &text[start..end],
            result: None,
            context: None,
        }
    }
    fn bind(&mut self, context: &'b mut boa_engine::Context) {
        self.context = Some(context);
    }
    fn eval(&mut self) {
        match self
            .context
            .as_mut()
            .unwrap()
            .eval(Source::from_bytes(self.code))
        {
            Ok(res) => {
                self.result = Some(res);
            }
            Err(e) => {
                panic!("Error at context evaluation {:#}", e);
            }
        }
    }
    fn replace(&mut self, text: &mut String) {
        let context = self.context.as_mut().expect("Should call bind first");
        let result = self.result.as_ref().expect("Sould call eval first");
        let Self { start, end, .. } = *self;
        let parsed_code = result.to_string(context).unwrap().to_std_string_escaped();
        text.replace_range(start - 2..=end, &parsed_code);
    }
}

#[derive(Default)]
pub struct ProcessingContext(boa_engine::Context);

impl From<&Option<&Context>> for ProcessingContext {
    fn from(value: &Option<&Context>) -> Self {
        let mut context = boa_engine::Context::default();
        if let Some(value) = value {
            #[allow(suspicious_double_ref_op)]
            for (key, value) in value.deref().deref() {
                context
                    .register_global_property(
                        js_string!(key.to_string()),
                        value.clone(),
                        Attribute::READONLY,
                    )
                    .unwrap();
            }
        }
        ProcessingContext(context)
    }
}

impl ProcessingContext {
    pub fn parse(&mut self, text: &str) -> Value {
        let re = regex::Regex::new(r"\$(\w+)").unwrap();
        let text = re
            .replace_all(text, |cap: &regex::Captures| format!("${{{:}}}", &cap[1]))
            .to_string();
        let mut processing_contexts = find_processing_contexts(&text);
        let is_processing_context_only = processing_contexts.len() == 1
            && *text == format!("${{{:}}}", processing_contexts[0].code); // ignoring "}"
        if is_processing_context_only {
            let mut item = processing_contexts.pop().unwrap();
            item.bind(&mut self.0);
            item.eval();
            Value::from(item.result.unwrap())
        } else {
            let mut result = text.to_owned();
            for mut item in processing_contexts.into_iter().rev() {
                item.bind(&mut self.0);
                item.eval();
                item.replace(&mut result);
            }
            Value::String(result)
        }
    }
    pub fn bind(&mut self, value: &Value) {
        let map = match value.clone() {
            values if values.is_mapping() => values,
            value => Value::force_mapping(value),
        }
        .as_mapping()
        .unwrap()
        .to_owned();
        for (key, value) in map {
            self.0
                .register_global_property(js_string!(key), value, Attribute::READONLY)
                .unwrap();
        }
    }
}

fn find_processing_contexts(text: &str) -> Vec<ProcessingContextCode<'_, '_>> {
    let mut head = 0;
    let last = text.len() - 1;
    let mut contexts = Vec::new();
    while head < last {
        if let Some(start) = text[head..].find("${") {
            head += start + 1;
            if let Some(end) = find_matching_curly_brace(text, head) {
                contexts.push(ProcessingContextCode::build(head + 1, end, text));
                head = end + 1;
            }
        } else {
            break;
        }
    }
    contexts
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_processing_contexts_test_1() {
        let text = "1 + 2 = ${a + b}";
        let contexts = find_processing_contexts(text);
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts[0].code, "a + b");
    }

    #[test]
    fn find_processing_contexts_test_2() {
        // let text = "1 + 2 = ${a + b} * ${a == 1} ? {a: 2} : ((x)=>{`2*(${x+1})`})(b)}";
        let text = "1 + 2 = ${a + b} * ${a == 1}";
        let contexts = find_processing_contexts(text);
        assert_eq!(contexts.len(), 2);
        assert_eq!(contexts[0].code, "a + b");
        assert_eq!(contexts[1].code, "a == 1");
    }

    #[test]
    fn find_processing_contexts_test_complex() {
        let text = "1 + 2 = ${a + b} * ${a == 1 ? {a: 2} : ((x)=>{`2*(${x+1})`})(b)}";
        let contexts = find_processing_contexts(text);
        assert_eq!(contexts.len(), 2);
        assert_eq!(contexts[0].code, "a + b");
        assert_eq!(
            contexts[1].code,
            "a == 1 ? {a: 2} : ((x)=>{`2*(${x+1})`})(b)"
        );
    }

    #[test]
    fn parse_test() {
        let mut text = "1 + 2 = ${((x)=>x + 2)(1)}".to_string();
        let mut pc = ProcessingContext::default();
        let value = pc.parse(&mut text);
        let text = value.as_str().unwrap();
        assert_eq!(text, "1 + 2 = 3");
    }
}
