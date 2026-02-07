Let's debug this step by step. Add logging to see where it's failing:

```rust
async fn connection_loop(stream: UnixStream, mut sender: Sender<Message>) -> Result<()> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut message = String::new();

    match reader.read_line(&mut message).await {
        Ok(0) => {
            log::debug!("Connection closed");
            return Ok(());
        }
        Ok(n) => {
            log::debug!("Received {} bytes: {:?}", n, message);

            let args = std::iter::once("persway")
                .chain(message.trim().split_ascii_whitespace())
                .collect::<Vec<_>>();

            log::debug!("Parsed args: {:?}", args);

            match Args::try_parse_from(args) {
                Ok(args) => {
                    log::debug!("Command parsed successfully: {:?}", args.command);
                    sender.send(Message::CommandEvent(args.command)).await?;
                    log::debug!("Command sent to handler");
                    writer.write_all(b"success\n").await?;
                }
                Err(e) => {
                    log::error!("Invalid command: {e}");
                    writer.write_all(b"fail: invalid command\n").await?;
                }
            }
        }
        Err(e) => log::error!("Socket read error: {e}"),
    }
    Ok(())
}
```

Then run the daemon and try the command again:

```bash
# In one terminal with logging enabled
RUST_LOG=debug persway daemon
```

# In another terminal

```bash
persway change-layout stack-main
```

Also verify:

Did you fix the \\n to \n issue? (Check both lines)

How are you sending the command? Are you using a separate persway binary that
connects to the socket?

Can you show me your top-level Args struct definition?

The logs will tell us if:

The message is being received

The parsing is succeeding

The command is being sent through the channel

The handler is receiving and processing it

What do the logs show when you run the command?
