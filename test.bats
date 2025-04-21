# This test suite is using BATS
# https://github.com/bats-core/bats-core

cli=./target/debug/emojfuscate

@test "text: direct argument to direct argument" {
  original_message="Karin är söt <3"
  emojfuscated=$($cli encode "$original_message")
  roundtrip_message=$($cli decode "$emojfuscated")

  [ "$roundtrip_message" = "$original_message" ]
}

@test "text: direct argument to stdin" {
  original_message="Karin är söt <3"
  emojfuscated=$($cli encode "$original_message")
  roundtrip_message=$(echo "$emojfuscated" | $cli decode -)

  [ "$roundtrip_message" = "$original_message" ]
}

@test "text: stdin to direct argument" {
  original_message="Karin är söt <3"
  emojfuscated=$(echo -n "$original_message" | $cli encode -)
  roundtrip_message=$($cli decode "$emojfuscated")

  [ "$roundtrip_message" = "$original_message" ]
}

@test "text: stdin to stdin" {
  original_message="Karin är söt <3"
  emojfuscated=$(echo -n "$original_message" | $cli encode -)
  roundtrip_message=$(echo "$emojfuscated" | $cli decode -)

  [ "$roundtrip_message" = "$original_message" ]
}



@test "uuid: direct argument to direct argument" {
  original_message="29f5467d-b8bf-4878-b0c3-57c72f9b9cd1"
  emojfuscated=$($cli encode -d uuid "$original_message")
  roundtrip_message=$($cli decode -d uuid "$emojfuscated")

  [ "$roundtrip_message" = "$original_message" ]
}

@test "uuid: direct argument to stdin" {
  original_message="29f5467d-b8bf-4878-b0c3-57c72f9b9cd1"
  emojfuscated=$($cli encode -d uuid "$original_message")
  roundtrip_message=$(echo "$emojfuscated" | $cli decode -d uuid -)

  [ "$roundtrip_message" = "$original_message" ]
}

@test "uuid: stdin to direct argument" {
  original_message="29f5467d-b8bf-4878-b0c3-57c72f9b9cd1"
  emojfuscated=$(echo -n "$original_message" | $cli encode -d uuid -)
  roundtrip_message=$($cli decode -d uuid "$emojfuscated")

  [ "$roundtrip_message" = "$original_message" ]
}

@test "uuid: stdin to stdin" {
  original_message="29f5467d-b8bf-4878-b0c3-57c72f9b9cd1"
  emojfuscated=$(echo -n "$original_message" | $cli encode -d uuid -)
  roundtrip_message=$(echo "$emojfuscated" | $cli decode -d uuid -)

  [ "$roundtrip_message" = "$original_message" ]
}

@test "uuid: handle whitespace" {
  original_message="29f5467d-b8bf-4878-b0c3-57c72f9b9cd1"
  roundtrip_message=$(echo "$original_message" | $cli encode -d uuid - | $cli decode -d uuid -)

  [ "$roundtrip_message" = "$original_message" ]
}



@test "hexadecimal: direct argument to direct argument" {
  original_message="29f5467db8bf4878b0c357c72f9b9cd1"
  emojfuscated=$($cli encode -d hexadecimal "$original_message")
  roundtrip_message=$($cli decode -d hexadecimal "$emojfuscated")

  [ "$roundtrip_message" = "$original_message" ]
}

@test "hexadecimal: direct argument to stdin" {
  original_message="29f5467db8bf4878b0c357c72f9b9cd1"
  emojfuscated=$($cli encode -d hexadecimal "$original_message")
  roundtrip_message=$(echo "$emojfuscated" | $cli decode -d hexadecimal -)

  [ "$roundtrip_message" = "$original_message" ]
}

@test "hexadecimal: stdin to direct argument" {
  original_message="29f5467db8bf4878b0c357c72f9b9cd1"
  emojfuscated=$(echo -n "$original_message" | $cli encode -d hexadecimal -)
  roundtrip_message=$($cli decode -d hexadecimal "$emojfuscated")

  [ "$roundtrip_message" = "$original_message" ]
}

@test "hexadecimal: stdin to stdin" {
  original_message="29f5467db8bf4878b0c357c72f9b9cd1"
  emojfuscated=$(echo -n "$original_message" | $cli encode -d hexadecimal -)
  roundtrip_message=$(echo "$emojfuscated" | $cli decode -d hexadecimal -)

  [ "$roundtrip_message" = "$original_message" ]
}
