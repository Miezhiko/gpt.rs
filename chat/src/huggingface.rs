use crate::types::Generator;

use inline_python::{ python, Context };
use std::panic::catch_unwind;

use anyhow::bail;

use async_trait::async_trait;

pub struct HuggingFaceGenerator {
  name: &'static str,
  enabled: bool,
  enabled_for_multigen: bool,
  model: &'static str
}

#[async_trait]
impl Generator for HuggingFaceGenerator {
  fn name<'a>(&self) -> &'a str { self.name }
  fn enabled(&self) -> bool { self.enabled }
  fn enabled_for_multigen(&self) -> bool { self.enabled_for_multigen }
  async fn call(&self, prompt: &str, _fmode: bool, _personality: &str) -> anyhow::Result<String> {
    #[allow(clippy::blocks_in_conditions)]
    match catch_unwind(|| {
      let c = Context::new();
      c.set("prompt", prompt);
      c.set("model", self.model);
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
}

pub const HUGGING_FACE_GENERATOR_PHI: HuggingFaceGenerator = HuggingFaceGenerator {
  name: "HuggingFaceGeneratorPhi",
  enabled: true,
  enabled_for_multigen: true,
  model: "microsoft/Phi-3-mini-4k-instruct"
};

pub const HUGGING_FACE_GENERATOR_MIXTRAL: HuggingFaceGenerator = HuggingFaceGenerator {
  name: "HuggingFaceGeneratorMixtral",
  enabled: true,
  enabled_for_multigen: true,
  model: "mistralai/Mistral-7B-Instruct-v0.2"
};

pub const HUGGING_FACE_GENERATOR_GEMMA: HuggingFaceGenerator = HuggingFaceGenerator {
  name: "HuggingFaceGeneratorGemma",
  enabled: true,
  enabled_for_multigen: true,
  model: "google/gemma-1.1-7b-it"
};

pub const HUGGING_FACE_GENERATOR_NOUS_RESEARCH: HuggingFaceGenerator = HuggingFaceGenerator {
  name: "NousResearch",
  enabled: true,
  enabled_for_multigen: false,
  model: "NousResearch/Nous-Hermes-2-Mixtral-8x7B-DPO"
};

pub const HUGGING_FACE_GENERATOR_YI: HuggingFaceGenerator = HuggingFaceGenerator {
  name: "Yi",
  enabled: true,
  enabled_for_multigen: false,
  model: "01-ai/Yi-1.5-34B-Chat"
};

#[cfg(test)]
mod huggingface_tests {
  use super::*;
  #[tokio::test]
  async fn huggingface_tests() {
    let gen = HUGGING_FACE_GENERATOR_MIXTRAL;
    let chat_response =
      gen.call("what gpt version you use?", true, "Fingon").await;
    assert!(chat_response.is_ok());
    assert!(!chat_response.unwrap().contains("is not working"));
  }
}
