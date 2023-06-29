pub fn execute(cmd: Args) -> Result<(), Error> {
    let mut read_as_json = BTreeMap::new();
    let reads = !cmd.get.is_empty();

    for item in cmd.get {
        let mut ptr = Some(doc.as_item());
        for piece in item.split('.') {
            ptr = ptr.as_ref().and_then(|x| x.get(piece));
        }
    }
}
/*
https://github.com/mitsuhiko/rye/blob/4afe320133335b6798e25e26df079ec4228edd0b/rye/src/cli/config.rs#L69
*/
