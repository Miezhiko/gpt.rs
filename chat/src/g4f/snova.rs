use crate::{
  types::Generator,
  personality::get_personality,
  constants
};

use inline_python::{ python, Context };
use std::panic::catch_unwind;

use anyhow::bail;

use std::collections::VecDeque;

use once_cell::sync::Lazy;

use tokio::sync::Mutex;

use async_trait::async_trait;

use chat_utils::help::lang;

static MSGHIST: Lazy<Mutex<VecDeque<(String, String)>>> =
  Lazy::new(|| Mutex::new( VecDeque::with_capacity(1) ));

pub struct SnovaGenerator;

#[async_trait]
impl Generator for SnovaGenerator {
  fn name<'a>( &self ) -> &'a str {
    "Snova"
  }
  fn enabled( &self ) -> bool {
    true
  }
  fn enabled_for_multigen( &self ) -> bool {
    true
  }
  async fn call( &self
               , prompt: &str
               , fmode: bool
               , personality: &str )
    -> anyhow::Result<String> {
    let mut msg_lock = MSGHIST.lock().await;
    let tmp_msg = msg_lock.as_slices();
    let russian = lang::is_russian(prompt);
    #[allow(clippy::blocks_in_conditions)]
    match catch_unwind(|| {
      let c = Context::new();
      c.set("prompt", prompt);
      c.set("old_messages", tmp_msg);
      c.set("is_russian", russian);
      c.set("fmode", fmode);
      c.set("PERSONALITY", get_personality(personality));
      c.run(python! {
        import sys
        import os
        import g4f
  
        result = ""
        if fmode:
          systemContext = PERSONALITY
        else:
          systemContext = "You are a helpful assistant"
        if is_russian:
          systemContext += ", you reply in Russian, you don't provide Translation"
        else:
          systemContext += ", you reply in English"
        messages = [{"role": "system", "content": systemContext}]
        if not fmode and old_messages:
          for tup in old_messages:
            if tup and len(tup) == 2:
              messages.append({"role": "user", "content": tup[0]})
              messages.append({"role": "assistant", "content": tup[1]})
        try:
          messages.append({"role": "user", "content": prompt})
          rspns = g4f.ChatCompletion.create( model="Meta-Llama-3.1-8B-Instruct", messages=messages
                                           , stream=False, auth="jwt"
                                           , provider=g4f.Provider.Snova )
          if not rspns:
            result = "Snova: Sorry, I can't generate a response right now."
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
      Ok((r,m_with_trash)) => {
        if r {
          let re = regex::Regex::new(r"\$@\$(.*?)\$@\$").unwrap();
          let m = re.replace_all(m_with_trash.as_str(), "");
          if !m.is_empty() {
            if msg_lock.len() == msg_lock.capacity() {
              msg_lock.pop_front();
            }
            if (prompt.len() + m.len()) < constants::HISTORY_LIMIT {
              msg_lock.push_back((prompt.to_string(), m.to_string()));
            }
          }
          Ok(m.to_string())
        } else {
          bail!("No tokens generated: {:?}", m_with_trash)
        }
      }, Err(_) => { bail!("Failed to to use Snova now!") }
    }
  }
}

#[cfg(test)]
mod snova_tests {
  use super::*;
  #[tokio::test]
  async fn snova_test() {
    let gen = SnovaGenerator;
    let chat_response =
      gen.call("what gpt version you use?", true, "Fingon").await;
    assert!(chat_response.is_ok());
    assert!(!chat_response.unwrap().contains("is not working"));
  }
}
