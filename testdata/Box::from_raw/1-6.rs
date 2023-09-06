unsafe extern "C" fn off_thread_compilation_callback(
    token: *mut OffThreadToken,
    callback_data: *mut c_void,
) {
    let mut context = Box::from_raw(callback_data as *mut OffThreadCompilationContext);
    // let token = OffThreadCompilationToken(token);

    // let url = context.url.clone();
    // let final_url = context.final_url.clone();
    // let script_element = context.script_element.clone();
    // let script_kind = context.script_kind.clone();
    // let script = replace(&mut context.script_text, String::new());
    // let fetch_options = context.fetch_options.clone();

    // Continue with <https://html.spec.whatwg.org/multipage/#fetch-a-classic-script>
    // let _ = context.task_source.queue_with_canceller(
    //     task!(off_thread_compile_continue: move || {
    //         let elem = script_element.root();
    //         let global = elem.global();
    //         let cx = GlobalScope::get_cx();
    //         let _ar = enter_realm(&*global);

    //         let compiled_script = FinishOffThreadStencil(*cx, token.0, ptr::null_mut());

    //         let load = if compiled_script.is_null() {
    //             Err(NetworkError::Internal(
    //                 "Off-thread compilation failed.".into(),
    //             ))
    //         } else {
    //             let script_text = DOMString::from(script);
    //             let code = SourceCode::Compiled(CompiledSourceCode {
    //                 source_code: compiled_script,
    //                 original_text: Rc::new(script_text),
    //             });

    //             Ok(ScriptOrigin {
    //                 code,
    //                 url: final_url,
    //                 external: true,
    //                 fetch_options: fetch_options,
    //                 type_: ScriptType::Classic,
    //             })
    //         };

    //         finish_fetching_a_classic_script(&*elem, script_kind, url, load);
    //     }),
    //     &context.canceller,
    // );
}
// https://github.com/servo/servo/blob/66abb1dfc49ab1067b7a3d7bc2d40b772f949bcd/components/script/dom/htmlscriptelement.rs#L99