use clap::Parser;
use hex;
use std::io;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::process::ExitCode;
use uuid::Uuid;

use emojfuscate::{
    ByteInSequence, Demojfuscate, Emojfuscate, EmojfuscateByteStream, FromEmojiError,
    IsEmojiRepresentation,
};
mod cli_args;
mod hex_stream;

fn main() -> ExitCode {
    let cli = cli_args::Cli::parse();

    let unwrapped_std_in = io::stdin().bytes().map(|b| b.unwrap());

    let mut stream = BufWriter::new(io::stdout());

    match &cli.command {
        cli_args::Commands::Encode {
            line_break,
            data_type,
            input,
        } => {
            match &data_type {
                cli_args::DataType::UUID => {
                    let uuid = match input.as_str() {
                        "-" => Uuid::parse_str(
                            std::str::from_utf8(&unwrapped_std_in.collect::<Vec<u8>>()).unwrap(),
                        )
                        .unwrap(),
                        uuid_string => Uuid::parse_str(uuid_string).unwrap(),
                    };

                    for emoji in uuid.emojfuscate_stream() {
                        stream.write(emoji.to_string().as_bytes()).unwrap();
                    }
                }
                cli_args::DataType::Hexadecimal => match input.as_str() {
                    "-" => {
                        for emoji in
                            hex_stream::HexStream::new(unwrapped_std_in).emojfuscate_stream()
                        {
                            stream.write(emoji.to_string().as_bytes()).unwrap();
                        }
                    }
                    some_string => {
                        for emoji in
                            hex_stream::HexStream::new(some_string.bytes()).emojfuscate_stream()
                        {
                            stream.write(emoji.to_string().as_bytes()).unwrap();
                        }
                    }
                },
                cli_args::DataType::Text => match input.as_str() {
                    "-" => {
                        for emoji in unwrapped_std_in.emojfuscate_byte_stream() {
                            stream.write(emoji.to_string().as_bytes()).unwrap();
                        }
                    }
                    some_string => {
                        stream
                            .write(some_string.bytes().emojfuscate().as_bytes())
                            .unwrap();
                    }
                },
            }

            if *line_break {
                stream.write("\n".as_bytes()).unwrap();
            }
        }
        cli_args::Commands::Decode {
            line_break,
            data_type,
            input,
        } => {
            let unit_or_error = match &data_type {
                cli_args::DataType::UUID => {
                    let r_uuid: Result<Uuid, FromEmojiError> = match input.as_str() {
                        "-" => unwrapped_std_in.demojfuscate(),
                        some_string => some_string.bytes().demojfuscate(),
                    };

                    r_uuid.map(|uuid| {
                        stream
                            .write(
                                uuid.hyphenated()
                                    .encode_lower(&mut Uuid::encode_buffer())
                                    .as_bytes(),
                            )
                            .unwrap();
                    })
                }
                cli_args::DataType::Hexadecimal => match input.as_str() {
                    "-" => {
                        let mut byte_stream = unwrapped_std_in.demojfuscate_byte_stream();

                        // a for loop would be simpler but you can't break out of it with a value
                        loop {
                            match byte_stream.next() {
                                Some(Ok(ByteInSequence::Byte(byte))) => {
                                    stream.write(hex::encode(&[byte]).as_bytes()).unwrap();
                                }
                                Some(Ok(_)) => continue,
                                Some(Err(err)) => break Err(err),
                                None => break Ok(()),
                            }
                        }
                    }
                    some_string => some_string.demojfuscate().map(|vec: Vec<u8>| {
                        stream
                            .write(hex::encode(vec.as_slice()).as_bytes())
                            .unwrap();
                    }),
                },
                cli_args::DataType::Text => match input.as_str() {
                    "-" => {
                        let mut byte_stream = unwrapped_std_in.demojfuscate_byte_stream();

                        // a for loop would be simpler but you can't break out of it with a value
                        loop {
                            match byte_stream.next() {
                                Some(Ok(ByteInSequence::Byte(byte))) => {
                                    stream.write(&[byte]).unwrap();
                                }
                                Some(Ok(_)) => continue,
                                Some(Err(err)) => break Err(err),
                                None => break Ok(()),
                            }
                        }
                    }
                    some_string => some_string.demojfuscate().map(|string: String| {
                        stream.write(string.as_bytes()).unwrap();
                    }),
                },
            };

            match unit_or_error {
                Ok(()) => {
                    if *line_break {
                        stream.write("\n".as_bytes()).unwrap();
                    }
                }
                Err(err) => {
                    io::stdout().flush().unwrap();

                    let error_message = match err {
                        FromEmojiError::NotEnoughEmoji => {
                            "Not enough emoji in input to construct text".to_string()
                        }
                        FromEmojiError::UnexpectedInput(error_message) => {
                            error_message
                        }
                        FromEmojiError::InvalidUtf8 => {
                            "The input bytes are not valid UTF-8".to_string()
                        }
                        FromEmojiError::InputIsNotAnEmoji(error_message) => {
                            error_message
                        }
                        FromEmojiError::MissingSequenceStart => {
                            "Expected a sequence start emoji when decoding data with variable length".to_string()
                        }
                        FromEmojiError::UnexpectedSequenceStart(err) => {
                            format!("Unexpected a sequence start emoji {}", err)
                        }
                        FromEmojiError::UnexpectedSequenceEnd => {
                            "Unexpected a sequence end emoji when decoding data".to_string()
                        }
                    };

                    eprintln!("{}", error_message);

                    return ExitCode::FAILURE;
                }
            }
        }
    };

    io::stdout().flush().unwrap();

    return ExitCode::SUCCESS;
}
