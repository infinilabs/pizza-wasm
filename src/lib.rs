#![no_std]
// Or
//
// ```rust
// #[allow(unused_assignments)]
// default_analyzer_name = analyzer_name
// ```
//
// won't be allowed
#![feature(stmt_expr_attributes)]

extern crate alloc;

use alloc::boxed::Box;
#[cfg(feature = "debug")]
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use cfg_if::cfg_if;
use core::fmt::Display;
use core::fmt::Formatter;
use hashbrown::HashMap;
use pizza_engine as engine;
use pizza_engine::document::DocID;
use pizza_engine::document::DraftDoc;
use pizza_engine::document::FieldType;
use pizza_engine::document::FieldValue;
use pizza_engine::document::Property;
use pizza_engine::search::OriginalQuery;
use pizza_engine::search::QueryContext;
use pizza_engine::search::Searcher;
use pizza_engine::store::MemoryStore;
use pizza_engine::Engine;
use pizza_engine::EngineBuilder;
use wasm_bindgen::prelude::wasm_bindgen;

use pizza_engine::analysis::AnalyzerConfig;

use pizza_engine::search::query::Operator;
#[cfg(feature = "stemmers")]
use pizza_stemmers::algorithms;
#[cfg(feature = "stemmers")]
use pizza_stemmers::StemmerTokenizer;
use serde_json::Value;
use wasm_bindgen::JsValue;

#[cfg(feature = "jieba")]
use pizza_jieba::JiebaTokenizer;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub struct Pizza {
    engine: Engine<MemoryStore>,
    searcher: Searcher<MemoryStore>,
}

impl Pizza {
    pub fn search(&self, query_context: &QueryContext) -> JsValue {
        let result = self.searcher.parse_and_query(query_context, &());

        let result = match result {
            Ok(o) => o,
            Err(e) => {
                let e_str = e.to_string();
                web_sys::console::log_1(&JsValue::from_str(&e_str));
                todo!()
            }
        };

        #[cfg(feature = "debug")]
        {
            let mut output = format!("Total Hits: {}\n", &result.total_hits);

            // Step 5: Iterate through the hits and append details to the output string
            if let Some(hits) = &result.hits {
                for hit in hits {
                    output.push_str(&format!(
                        "- Document ID: {}, Score: {:?}, Source: {:?}\n",
                        hit.id, hit.score, hit.fields
                    ));
                }
            }

            if let Some(explain) = &result.explains {
                output.push_str(&format!("Explanations: {}\n", explain));
            }
            web_sys::console::log_1(&output.into());
        }

        let serialized_result = serde_json::to_string(&result)
            .unwrap_or_else(|e| panic!("failed to serialize result due to {}", e));
        JsValue::from_str(&serialized_result)
    }
}

#[wasm_bindgen]
impl Pizza {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Pizza {
        //start with 15kb
        let mut builder = EngineBuilder::new(); //19kb

        let memory_store = MemoryStore::new();
        builder.set_data_store(memory_store); //30kb

        // It is not unused if features `stemmers` and `jieba` are disabled
        #[allow(unused_assignments)]
        let mut default_analyzer_name = "standard";

        //init stemmers
        #[cfg(feature = "stemmers")]
        {
            let mut analyzers = HashMap::new();
            let token_filter_name = "snowball_english_porter_2";
            let tokenizer = StemmerTokenizer::new(algorithms::english_porter_2);
            builder.register_analysis_tokenizer(token_filter_name, Box::new(tokenizer));

            let mut analyzer = AnalyzerConfig::new();
            analyzer.set_tokenizer("standard");
            analyzer.add_token_filter("lowercase");
            analyzer.add_token_filter(token_filter_name);

            let analyzer_name = "standard_with_english_stemmer";
            analyzers.insert(analyzer_name, analyzer);
            builder.set_analyzer_configs(analyzers);
            // It is only unused if features `stemmers` and `jieba` are both enabled
            #[allow(unused_assignments)]
            default_analyzer_name = analyzer_name
        }

        //init jieba
        #[cfg(feature = "jieba")]
        {
            let mut analyzers = HashMap::new();
            let tokenizer = JiebaTokenizer::new();
            let tokenizer_name = "jieba";
            builder.register_analysis_tokenizer(tokenizer_name, Box::new(tokenizer));
            let mut analyzer = AnalyzerConfig::new();
            analyzer.set_tokenizer(tokenizer_name);
            analyzer.add_token_filter("lowercase");
            analyzers.insert(tokenizer_name, analyzer);
            default_analyzer_name = tokenizer_name
        }

        //init schema
        let mut schema = engine::document::Schema::new();
        schema
            .add_property("title", Property::as_text(Some(default_analyzer_name)))
            .unwrap();
        schema
            .add_property("content", Property::as_text(Some(default_analyzer_name)))
            .unwrap();
        schema
            .add_property("category", Property::as_text(Some(default_analyzer_name)))
            .unwrap();
        schema
            .add_property(
                "subcategory",
                Property::as_text(Some(default_analyzer_name)),
            )
            .unwrap();
        schema
            .add_property("tags", Property::as_text(Some(default_analyzer_name)))
            .unwrap();
        schema
            .add_property(
                "url",
                Property::builder(FieldType::Keyword).index(false).build(),
            )
            .unwrap(); //42kb

        schema.freeze(); //95kb
        builder.set_schema(schema); //95kb

        let engine = builder.build().expect("failed to build the engine"); //144kb
        engine.start(); //385kb / 359 on opt-level = 'z' / 347kb on no locales

        let searcher = engine.acquire_searcher(); //347kb

        //basic setup: 355kb

        //enable stemmers //355kb->371kb
        //enable jieba //355kb->6.6M

        Pizza { engine, searcher }
    }

    // the data is in plain text, split item per each line
    #[cfg(feature = "load_text_lines")]
    pub async fn load_text_lines(&mut self, data: &str) -> bool {
        let mut writer = self.engine.acquire_writer();

        let mut id = 0;
        for line in data.lines() {
            id += 1;

            let fields = {
                let mut m = HashMap::new();
                m.insert("title".to_string(), FieldValue::Text(line.into()));
                m
            };

            let doc = DraftDoc::new_with_id_and_fields(DocID::ID(id), fields);

            writer.create_document(doc).await.unwrap();
        }

        writer.flush().unwrap();
        writer.commit().unwrap();

        true
    }

    // the data is in json array, eg: [{doc},{doc}]
    // after enable this feature: 355kb->428kb
    #[cfg(feature = "load_json_objects_array")]
    pub async fn load_json_objects_array(&mut self, data: &str) -> bool {
        let mut writer = self.engine.acquire_writer();

        let mut id = 0;
        // Parse the JSON string into a Vec of serde_json::Value
        let json_array: Vec<Value> = match serde_json::from_str(data) {
            Ok(array) => array,
            Err(err) => {
                cfg_if! {
                    if #[cfg(feature = "debug")] {
                        let error_message = format!("Failed to parse JSON: {:?}", err);
                        web_sys::console::log_1(&error_message.into());
                    }

                }

                // To "use" it so that we don't have an unused warning
                let _ = err;
                return false;
            }
        };

        // Iterate over each item in the JSON array
        for item in json_array {
            id += 1;

            // Ensure the item is an object and extract fields
            if let Some(obj) = item.as_object() {
                let mut fields = HashMap::new();

                // Insert each key-value pair from the JSON object into the fields map
                for (key, value) in obj.iter() {
                    let field_value = match value {
                        Value::String(s) => FieldValue::Text(s.to_string()),
                        Value::Number(n) => {
                            if let Some(i) = n.as_i64() {
                                FieldValue::Integer(i as i32)
                            } else if let Some(f) = n.as_f64() {
                                FieldValue::Float(f as f32)
                            } else {
                                continue;
                            }
                        }
                        Value::Bool(b) => FieldValue::Boolean(*b),
                        _ => continue, // Skip unsupported types
                    };

                    fields.insert(key.clone(), field_value);
                }

                let doc = DraftDoc::new_with_id_and_fields(DocID::ID(id), fields);

                #[cfg(feature = "debug")]
                {
                    let message = format!("Add document: {:?}", doc);
                    web_sys::console::log_1(&message.into());
                }

                writer.create_document(doc).await.unwrap();
            } else {
                #[cfg(feature = "debug")]
                {
                    let error_message = format!("Expected JSON object, found: {:?}", item);
                    web_sys::console::log_1(&error_message.into());
                }
                return false;
            }
        }

        writer.flush().unwrap();
        writer.commit().unwrap();

        true
    }

    #[cfg(feature = "query_string")]
    pub fn search_by_query_string(&self, query_string: &str) -> JsValue {
        let original_query = OriginalQuery::QueryString(query_string.to_string());

        let mut query_context = QueryContext::new(original_query, true);
        query_context.support_wildcard_in_field_name = true;
        query_context.default_operator = Operator::Or;
        query_context.default_field = "*".into();

        self.search(&query_context)
    }

    #[cfg(feature = "query_string")]
    pub fn advanced_search_by_query_string(
        &self,
        default_field: &str,
        query_string: &str,
        operator: &str,
        from: usize,
        size: usize,
        explain: bool,
    ) -> JsValue {
        let original_query = OriginalQuery::QueryString(query_string.to_string());

        let mut query_context = QueryContext::new(original_query, explain);
        query_context.support_wildcard_in_field_name = true;
        query_context.from = from;
        query_context.size = size;

        if operator.trim().to_uppercase() == "AND" {
            query_context.default_operator = Operator::And;
        } else {
            query_context.default_operator = Operator::Or;
        }
        query_context.default_field = default_field.into();

        self.search(&query_context)
    }

    // #[cfg(feature = "query_dsl")]
    // pub fn search_by_query_dsl(&self, query_dsl: &str) -> String {
    //     let original_query = OriginalQuery::QueryDSL(query_dsl.to_string());
    //
    //     let mut query_context = QueryContext::new(original_query, true);
    //     query_context.support_wildcard_in_field_name=true;
    //     query_context.default_operator = Operator::Or;
    //     query_context.default_field = "*".into();
    //
    //     self.search(&query_context).into()
    // }
}

impl Display for Pizza {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Pizza Engine (Wasm) v0.1")?;
        Ok(())
    }
}
