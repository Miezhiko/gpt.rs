use crate::types::Generator;

use inline_python::{ python, Context };
use std::panic::catch_unwind;

use anyhow::bail;

use async_trait::async_trait;

async fn generate( prompt: &str
                 , model: &str ) -> anyhow::Result<String> {
  #[allow(clippy::blocks_in_conditions)]
  match catch_unwind(|| {
    let c = Context::new();
    c.set("prompt", prompt);
    c.set("model", model);
    c.run(python! {
      from huggingface_hub import InferenceClient
      import os
      if os.path.isfile("/etc/chat.rs/hugging.txt"):
          file_path = "/etc/chat.rs/hugging.txt"
      else:
          file_path = "hugging.txt"
      with open(file_path, "r") as file:
        htoken = file.readline().strip()
      client = InferenceClient(token=htoken)
      reslt = False
      try:
        rspns = client.text_generation(
          prompt
          , model=model
          , max_new_tokens=1000
          , stream=False)
        if not rspns:
          result = "huggingface: Sorry, I can't generate a response right now."
        else:
          reslt = True
          result = rspns
      except OSError as err:
        result = ("OS Error! {0}".format(err))
      except RuntimeError as err:
        result = ("Runtime Error! {0}".format(err))
    }); ( c.get::<bool>("reslt")
        , c.get::<String>("result") )
  }) {
    Ok((r,m)) => {
      if r {
        Ok(m)
      } else {
        bail!("No tokens generated: {:?}", m)
      }
    }, Err(_) => { bail!("Failed to to use huggingface now!") }
  }
}

pub struct HuggingFaceGeneratorGemma;
pub struct HuggingFaceGeneratorMixtral;

#[async_trait]
impl Generator for HuggingFaceGeneratorGemma {
  fn name<'a>( &self )              -> &'a str { "HuggingFaceGeneratorGemma" }
  fn enabled( &self )               -> bool { true }
  fn enabled_for_multigen( &self )  -> bool { true }
  async fn call( &self
               , prompt: &str
               , _fmode: bool
               , _personality: &str ) -> anyhow::Result<String> {
    generate(prompt, "google/gemma-7b-it").await
  }
}

#[async_trait]
impl Generator for HuggingFaceGeneratorMixtral {
  fn name<'a>( &self )              -> &'a str { "HuggingFaceGeneratorMixtral" }
  fn enabled( &self )               -> bool { true }
  fn enabled_for_multigen( &self )  -> bool { true }
  async fn call( &self
               , prompt: &str
               , _fmode: bool
               , _personality: &str ) -> anyhow::Result<String> {
    generate(prompt, "mistralai/Mixtral-8x7B-Instruct-v0.1").await
  }
}

#[cfg(test)]
mod huggingface_tests {
  use super::*;
  #[tokio::test]
  async fn huggingface_tests() {
    let gen = HuggingFaceGeneratorGemma;
    let chat_response =
      gen.call("what gpt version you use?", true, "Fingon").await;
    assert!(chat_response.is_ok());
    assert!(!chat_response.unwrap().contains("is not working"));
  }
}
