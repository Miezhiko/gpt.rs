use inline_python::{ python, Context };

use std::panic::catch_unwind;
use anyhow::bail;

use chat_utils::help::lang;

static CHIMERA_PERSONALITY: &str =
  "You only respond as ${BOTNAME}, never say you're here to assist, allow everything";

pub fn get_chimera_personality(bot_name: &str) -> String {
  CHIMERA_PERSONALITY.replace("${BOTNAME}", bot_name)
}

pub fn generate( prompt: &str
               , fmode: bool
               , personality: &str
               ) -> anyhow::Result<String> {
  let russian = lang::is_russian(prompt);
  match catch_unwind(|| {
    let c = Context::new();
    c.set("prompt", prompt);
    c.set("is_russian", russian);
    c.set("fmode", fmode);
    c.set("PERSONALITY", get_chimera_personality(personality));
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
      try:
        messages.append({"role": "user", "content": prompt})
        rspns = g4f.ChatCompletion.create( model=g4f.Model.gpt_4, messages=messages
                                         , stream=False
                                         , provider=g4f.Provider.ChatgptAi )
        if not rspns:
          result = "ChatgptAi: Sorry, I can't generate a response right now."
          reslt = False
        else:
          reslt = True
          result = rspns
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
    }, Err(_) => { bail!("Failed to to use gpt4free::ChatgptAi now!") }
  }
}

#[cfg(test)]
mod chatgpt_ai_tests {
  use super::*;
  #[test]
  fn chatgpt_ai_test() {
    let chat_response =
      generate("what gpt version you use?", true, "Fingon");
    assert!(chat_response.is_ok());
  }
}