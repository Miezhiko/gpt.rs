# chat.rs

[![mawa](https://github.com/Miezhiko/chat.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Miezhiko/chat.rs/actions/workflows/ci.yml)

# Requirements:

- [huggingface](https://github.com/huggingface/huggingface_hub)
- [poe-api](https://github.com/ading2210/poe-api) (deprecated, dropped)
- [gpt4free](https://github.com/xtekky/gpt4free)

# Silly usage example

```rust
if let Ok(gpt4free_result) =
  chat::chat( payload, "bot name" ) {
    return Some(gpt4free_result);
  }
};
```

# Current state

```rust
static GENERATORS: Lazy<Vec<Arc<dyn Generator + Send + Sync>>> =
  Lazy::new(|| {
    vec![ ... ]
  });

pub async fn generate(msg: &str, bot_name: &str, fancy: bool) -> anyhow::Result<String> {
  for gen in &*GENERATORS {
    if let Ok(result) = gen.call(msg, fancy, bot_name).await {
      return Ok(result);
    }
  }
  Err( anyhow::anyhow!("All generators failed") )
}

pub async fn generate_all<'a>(msg: &str, bot_name: &str, fancy: bool)
                                -> Vec<(&'a str, anyhow::Result<String>)> {
  let genz = (&*GENERATORS).into_iter().map(
    |gen| async move { ( gen.name()
                       , gen.call(msg, fancy, bot_name).await )
                     }
  );
  future::join_all(genz).await
}
```
