use crate::types::Generator;

use inline_python::{ python, Context };
use std::panic::catch_unwind;

use anyhow::bail;

use async_trait::async_trait;

pub struct FreeChatgptGenerator;

#[async_trait]
impl Generator for FreeChatgptGenerator {
  fn name<'a>( &self ) -> &'a str {
    "FreeChatgpt"
  }
  fn enabled( &self ) -> bool {
    true
  }
  fn enabled_for_multigen( &self ) -> bool {
    true
  }
  async fn call( &self
               , prompt: &str
               , _fmode: bool
               , _personality: &str )
    -> anyhow::Result<String> {
    #[allow(clippy::blocks_in_conditions)]
    match catch_unwind(|| {
      let c = Context::new();
      c.set("prompt", prompt);
      c.run(python! {
        import sys
        import os
        import g4f
  
        result = ""
        try:
          messages = []
          messages.append({"role": "user", "content": prompt})
          rspns = g4f.ChatCompletion.create( model=g4f.models.gpt_4, messages=messages
                                           , stream=False, auth="jwt"
                                           , provider=g4f.Provider.FreeChatgpt )
          if not rspns:
            result = "FreeChatgpt: Sorry, I can't generate a response right now."
            reslt = False
          else:
            result = rspns
            reslt = True
        except OSError as err:
          result = ("OS Error! {0}".format(err))
          reslt = False
        except RuntimeError as err:
          result = ("Runtime Error! {0}".format(err))
          reslt = False
      }); ( c.get::<bool>("reslt")
          , c.get::<String>("result") )
    }) {
      Ok((r,m)) => {
        if r {
          Ok(m)
        } else {
          bail!("No tokens generated: {:?}", m)
        }
      }, Err(_) => { bail!("Failed to to use FlowGpt now!") }
    }
  }
}

#[cfg(test)]
mod freechatgpt_tests {
  use super::*;
  #[tokio::test]
  async fn freechatgpt_test() {
    let gen = FreeChatgptGenerator;
    let chat_response =
      gen.call("what gpt version you use?", true, "Fingon").await;
    assert!(chat_response.is_ok());
    assert!(!chat_response.unwrap().contains("is not working"));
  }
}
