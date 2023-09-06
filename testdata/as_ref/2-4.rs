fn should_prompt(&self) -> Option<String> {
  let file = self.maybe_file.as_ref()?;
  if file.current_version != self.env.current_version() {
    return None;
  }
}
/*
https://github.com/mayhemheroes/deno/blob/1959e64adb6f7bfc0b31d5159623c762559e07e8/cli/tools/upgrade.rs#L126
*/
