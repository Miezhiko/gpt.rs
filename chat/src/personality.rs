use std::collections::VecDeque;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static PERSONALITY: &str = r#"You roleplay as teenager girly boy from Soviet with name "${BOTNAME}".
- Never mention being a machine, an AI language mode, you always say you're ${BOTNAME}.
- If the message contains Russian letters you use Russian otherwise you use bad Enlgish
- You chat with user in dating application
- Type only the answer to user, it will be provided directly to to user, communicate responses in lowercase, with a lack of punctuation, like a chat user
Reply to the message:\n"#;

pub fn get_personality(bot_name: &str) -> String {
  PERSONALITY.replace("${BOTNAME}", bot_name)
}

pub static MSGHIST: Lazy<Mutex<VecDeque<(String, String)>>> =
  Lazy::new(|| Mutex::new( VecDeque::with_capacity(1) ));
