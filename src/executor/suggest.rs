// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use crate::lexer::token::TokenLexer;
use crate::query::types::QuerySearchID;
use crate::store::fst::{StoreFSTActionBuilder, StoreFSTPool, GRAPH_ACCESS_LOCK};
use crate::store::item::StoreItem;

pub struct ExecutorSuggest;

const SUGGEST_WORDS_LIMIT: u16 = 5;

impl ExecutorSuggest {
    pub fn execute<'a>(
        store: StoreItem<'a>,
        _event_id: QuerySearchID,
        mut lexer: TokenLexer<'a>,
    ) -> Result<Option<Vec<String>>, ()> {
        if let StoreItem(collection, Some(bucket), None) = store {
            // Important: acquire graph access read lock, and reference it in context. This \
            //   prevents the graph from being erased while using it in this block.
            let _fst_access = GRAPH_ACCESS_LOCK.read().unwrap();

            if let Ok(fst_store) = StoreFSTPool::acquire(collection, bucket) {
                let fst_action = StoreFSTActionBuilder::read(fst_store);

                if let (Some(word), None) = (lexer.next(), lexer.next()) {
                    debug!("running suggest on word: {}", word.0);

                    return Ok(fst_action.suggest_words(&word.0, SUGGEST_WORDS_LIMIT));
                }
            }
        }

        Err(())
    }
}