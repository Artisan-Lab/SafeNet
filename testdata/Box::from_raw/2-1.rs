fn drop(&mut self) {
    unsafe {
        llvm::LLVMRustContextSetDiagnosticHandler(self.llcx, self.old_handler);
        drop(Box::from_raw(self.data));
    }
}
// https://github.com/crablang/crab/blob/71a7fcd549f5db917f626c22db17f74b04a65f47/compiler/rustc_codegen_llvm/src/back/write.rs#L318