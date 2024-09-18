#![feature(async_closure)]

mod personality;
mod constants;

pub mod types;
pub mod g4f;
pub mod huggingface;

use crate::types::Generator;

use std::sync::Arc;

use futures::future;

use once_cell::sync::Lazy;

use rand::seq::SliceRandom;
use rand::thread_rng;

static GENERATORS: Lazy<Vec<Arc<dyn Generator + Send + Sync>>> =
  Lazy::new(|| {
    vec![ Arc::new( g4f::blackbox::BlackboxGenerator                  )
        , Arc::new( g4f::ddg::DDGGenerator                            )
        , Arc::new( g4f::nexra::NexraGenerator                        )
        , Arc::new( g4f::airforce::AirforceGenerator                  )
        , Arc::new( huggingface::HUGGING_FACE_GENERATOR_GEMMA         )
        , Arc::new( g4f::pizzagpt::PizzagptGenerator                  )
        , Arc::new( huggingface::HUGGING_FACE_GENERATOR_MIXTRAL       )
        , Arc::new( g4f::upstage::UpstageGenerator                    )
        ]
  });

pub async fn generate(msg: &str, bot_name: &str, fancy: bool) -> anyhow::Result<String> {
  let fmode =
    if fancy {
      ! (msg.contains("please")
      || msg.contains("пожалуйста")
      || msg.contains("Please")
      || msg.contains("Пожалуйста")
      || msg.contains("PLEASE"))
    } else {
      false
    };

  if fmode {
    let mut genrators = GENERATORS.clone();
    genrators.shuffle(&mut thread_rng());
    for gen in genrators {
      if gen.enabled() {
        if let Ok(result) = gen.call(msg, fmode, bot_name).await {
          if !result.contains("502: Bad gateway") {
            return Ok(result);
          }
        }
      }
    }
  } else {
    for gen in &*GENERATORS {
      if gen.enabled() {
        if let Ok(result) = gen.call(msg, fmode, bot_name).await {
          if !result.contains("502: Bad gateway") {
            return Ok(result);
          }
        }
      }
    }
  }

  Err( anyhow::anyhow!("All generators failed") )
}

pub async fn generate_all<'a>(msg: &str, bot_name: &str, fancy: bool)
                                -> Vec<(&'a str, anyhow::Result<String>)> {
  let fmode =
    if fancy {
      ! (msg.contains("please")
      || msg.contains("пожалуйста")
      || msg.contains("Please")
      || msg.contains("Пожалуйста")
      || msg.contains("PLEASE"))
    } else {
      false
    };

  let genz = GENERATORS.iter().map(
    |gen| async move { ( gen.name(),
      if gen.enabled_for_multigen()
             { gen.call(msg, fmode, bot_name).await }
        else { anyhow::Ok(String::from("disabled")) } )
    }
  );

  future::join_all(genz).await
}

pub async fn chat(msg: &str, bot_name: &str) -> anyhow::Result<String> {
  generate(msg, bot_name, true).await
}
