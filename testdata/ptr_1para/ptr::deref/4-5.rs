pub fn new(
    window: &Window,
    browsing_context_id: BrowsingContextId,
    top_level_browsing_context_id: TopLevelBrowsingContextId,
    frame_element: Option<&Element>,
    parent: Option<&WindowProxy>,
    opener: Option<BrowsingContextId>,
    creator: CreatorBrowsingContextInfo,
) -> DomRoot<WindowProxy> {
    unsafe {
        let WindowProxyHandler(handler) = window.windowproxy_handler();
        assert!(!handler.is_null());

        let cx = GlobalScope::get_cx();
        let window_jsobject = window.reflector().get_jsobject();
        assert!(!window_jsobject.get().is_null());
        assert_ne!(
            ((*get_object_class(window_jsobject.get())).flags & JSCLASS_IS_GLOBAL),
            0
        );
        let _ac = JSAutoRealm::new(*cx, window_jsobject.get());

        // Create a new window proxy.
        rooted!(in(*cx) let js_proxy = NewWindowProxy(*cx, window_jsobject, handler));
        assert!(!js_proxy.is_null());

        // Create a new browsing context.
        let current = Some(window.global().pipeline_id());
        let window_proxy = Box::new(WindowProxy::new_inherited(
            browsing_context_id,
            top_level_browsing_context_id,
            current,
            frame_element,
            parent,
            opener,
            creator,
        ));

        // The window proxy owns the browsing context.
        // When we finalize the window proxy, it drops the browsing context it owns.
        SetProxyReservedSlot(
            js_proxy.get(),
            0,
            &PrivateValue((&*window_proxy).as_void_ptr()),
        );

        // Notify the JS engine about the new window proxy binding.
        SetWindowProxy(*cx, window_jsobject, js_proxy.handle());

        // Set the reflector.
        debug!(
                "Initializing reflector of {:p} to {:p}.",
                window_proxy,
                js_proxy.get()
            );
        window_proxy.reflector.set_jsobject(js_proxy.get());
        DomRoot::from_ref(&*Box::into_raw(window_proxy))
    }
}
//https://github.com/servo/servo/blob/f11c6045e33a921f03223c313781586189309bd2/components/script/dom/windowproxy.rs#L214