#![allow(unused)]

fn main() {
    let got_entry = module.new_got_entry(addr);
    module.libcall_got_entries.insert(libcall, got_entry);
    let plt_entry = module.new_plt_entry(got_entry);
    module.libcall_plt_entries.insert(libcall, plt_entry);

}