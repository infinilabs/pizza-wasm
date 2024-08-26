#![no_std]
extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::string::ToString;
use cfg_if::cfg_if;
use core::fmt::Display;
use core::fmt::Formatter;
use alloc::boxed::Box;
use alloc::format;
use alloc::sync::Arc;
use hashbrown::HashMap;
use pizza_engine as engine;
use pizza_engine::dictionary::DatTermDict;
use pizza_engine::document::Document;
use pizza_engine::document::FieldValue;
use pizza_engine::document::Property;
use pizza_engine::store::MemoryStore;
use pizza_engine::Engine;
use pizza_engine::EngineBuilder;
use spin::RwLock;
use wasm_bindgen::prelude::wasm_bindgen;
use alloc::vec::Vec;
use pizza_engine::context::Snapshot;
use pizza_engine::search::OriginalQuery;
use pizza_engine::search::QueryContext;
use pizza_engine::search::Searcher;

use pizza_engine::analysis::AnalyzerConfig;

#[cfg(feature = "stemmers")]
use pizza_stemmers::algorithms;
#[cfg(feature = "stemmers")]
use pizza_stemmers::StemmerTokenizer;
use serde_json::Value;
use wasm_bindgen::__rt::std::panic::resume_unwind;
use wasm_bindgen::JsValue;
use pizza_engine::search::query::Operator;

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
    engine: Engine<DatTermDict>,
    snapshot: Snapshot,
    searcher: Searcher,
}

#[wasm_bindgen]
impl Pizza {
    pub fn new() -> Pizza {
        let mut builder = EngineBuilder::new();
        let mut analyzers = HashMap::new();

        //init stemmers
        #[cfg(feature = "stemmers")]
        {
            let tokenizer_name = "snowball_english_porter_2";
            let tokenizer = StemmerTokenizer::new(algorithms::english_porter_2);
            builder.register_plugin(tokenizer_name.into(), Box::new(tokenizer));

            let mut analyzer = AnalyzerConfig::new();
            analyzer.set_tokenizer(tokenizer_name);
            analyzer.add_token_filter("lowercase");
            analyzers.insert(tokenizer_name, analyzer);
        }

        //init jieba
        #[cfg(feature = "jieba")]
        {
            let tokenizer = JiebaTokenizer::new();
            let tokenizer_name = "jieba";
            builder.register_plugin(tokenizer_name, Box::new(tokenizer));
            let mut analyzer = AnalyzerConfig::new();
            analyzer.set_tokenizer(tokenizer_name);
            analyzer.add_token_filter("lowercase");
            analyzers.insert(tokenizer_name, analyzer);
        }

        builder.set_analyzer_configs(analyzers);

        let default_analyzer_name="snowball_english_porter_2";

        //init schema
        let mut schema = engine::document::Schema::new();
        schema
            .properties
            .add_property("title", Property::as_text(Some(default_analyzer_name)));
        schema
            .properties
            .add_property("content", Property::as_text(Some(default_analyzer_name)));
        schema
            .properties
            .add_property("category", Property::as_text(Some(default_analyzer_name)));
        schema
            .properties
            .add_property("subcategory", Property::as_text(Some(default_analyzer_name)));
        schema
            .properties
            .add_property("tags", Property::as_text(Some(default_analyzer_name)));
        schema
            .properties
            .add_property("url", Property::as_keyword().set_index(false).to_owned());

        schema.freeze();
        builder.set_schema(schema);

        builder.set_term_dict(DatTermDict::new(0));

        let memory_store = MemoryStore::new();
        builder.set_data_store(Arc::new(RwLock::new(memory_store)));

        let mut engine = builder.build();
        engine.start();

        let searcher = engine.acquire_searcher();
        let snapshot = engine.create_snapshot();

        Pizza {
            engine,
            snapshot,
            searcher,
        }
    }

    // the data is in plain text, split item per each line
    pub fn load_text_lines(&mut self, data: &str) -> bool {
        let mut writer = self.engine.acquire_writer();

        let mut id = 0;
        for line in data.lines() {
            id += 1;
            let doc = Document {
                id: id,
                key: None,
                score: None,
                fields: {
                    let mut m = HashMap::new();
                    m.insert("title".to_string(), FieldValue::Text(line.to_string()));
                    m
                },
            };

            writer.add_document(doc);
        }

        writer.flush();
        writer.commit();

        //update snapshot
        let snapshot = self.engine.create_snapshot();
        self.snapshot = snapshot;
        true
    }

    // the data is in json array, eg: [{doc},{doc}]
    pub fn load_json_objects_array(&mut self, data: &str) -> bool {
        let mut writer = self.engine.acquire_writer();

        let mut id = 0;
        // Parse the JSON string into a Vec of serde_json::Value
        let json_array: Vec<Value> = match serde_json::from_str(data) {
            Ok(array) => array,
            Err(err) => {
                let error_message = format!("Failed to parse JSON: {:?}", err);
                #[cfg(feature = "debug")]
                web_sys::console::log_1(&error_message.into());
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
                        Value::String(s) => FieldValue::Text(s.clone()),
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

                let doc = Document {
                    id,
                    key: None,
                    score: None,
                    fields,
                };

                #[cfg(feature = "debug")]
                {
                    let message=format!("Add document: {:?}", doc);
                    web_sys::console::log_1(&message.into());
                }

                writer.add_document(doc);
            } else {
                let error_message = format!("Expected JSON object, found: {:?}", item);
                #[cfg(feature = "debug")]
                web_sys::console::log_1(&error_message.into());
                return false
            }
        }

        writer.flush();
        writer.commit();

        //update snapshot
        let snapshot = self.engine.create_snapshot();
        self.snapshot = snapshot;
        true
    }

    #[cfg(feature = "query_string")]
    pub fn search_by_query_string(&self, query_string: &str) -> JsValue {
        // Step 1: Initialize the original query and query context
        let original_query = OriginalQuery::QueryString(query_string.to_string());

        let mut query_context = QueryContext::new(original_query, true);
        query_context.support_wildcard_in_field_name=true;
        query_context.default_operator = Operator::Or;
        query_context.default_field = "*".into();

        // Step 2: Execute the query
        let result = self
            .searcher
            .parse_and_query(&query_context, &self.snapshot)
            .unwrap();


        #[cfg(feature = "debug")]
        {
            // Step 4: Prepare the output string
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

            // Step 6: Optionally, include explanations if available
            if let Some(explain) = &result.explains {
                output.push_str(&format!("Explanations: {}\n", explain));
            }
            web_sys::console::log_1(&output.into());
        }

        // let json_string = serde_json::to_string(&result).unwrap();
        // 
        // json_string

        JsValue::from_serde(&result).unwrap()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Display for Pizza {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Hello world\n")?;
        Ok(())
    }
}
