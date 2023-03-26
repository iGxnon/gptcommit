use colored::Colorize;

pub(crate) fn print_help_openai_api_key() {
    println!(
                "{}",
            r#"OpenAI API key not found in config or environment.

Configure the OpenAI API key with the command:

    export GPTCOMMIT__OPENAI__API_KEY='sk-...'

Or add the following to your ~/.config/gptcommit/config.toml file:
```
model_provider = "openai"

[openai]
api_key = "sk-..."
```

Note: OPENAI_API_KEY will be deprecated in a future release. Please use GPTCOMMIT__OPENAI__API_KEY instead, or a config file.
"#
            .bold()
            .yellow(),
        );
}

pub(crate) fn exit_with_network_issue(region: Option<String>) {
    if let Some(region) = region {
        println!(
            r#"🤖 Your IP address is from {}, which is not in [supported regions of OpenAI]( https://platform.openai.com/docs/supported-countries )."#,
            region
        );
    } else {
        println!(
            r#"🤖 We were unable to check if your IP region is a [supported region of OpenAI]( https://platform.openai.com/docs/supported-countries )."#
        );
    }
    println!(
        r#"Please check your Internet connection, and ensure that you are accessing the API in a supported region of OpenAI,
or your account may get banned regardless of your GPT Plus subscription status or any remaining account balance."#
    );
    std::process::exit(0);
}
