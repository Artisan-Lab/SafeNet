pub(crate) fn markdown_links<R>(
    md: &str,
    preprocess_link: impl Fn(MarkdownLink) -> Option<R>,
) -> Vec<R> {
    if md.is_empty() {
        return vec![];
    }

    // FIXME: remove this function once pulldown_cmark can provide spans for link definitions.
    let locate = |s: &str, fallback: Range<usize>| unsafe {
        let s_start = s.as_ptr();
        let s_end = s_start.add(s.len());
        let md_start = md.as_ptr();
        let md_end = md_start.add(md.len());
        if md_start <= s_start && s_end <= md_end {
            let start = s_start.offset_from(md_start) as usize;
            let end = s_end.offset_from(md_start) as usize;
            MarkdownLinkRange::Destination(start..end)
        } else {
            MarkdownLinkRange::WholeLink(fallback)
        }
    };

    let span_for_link = |link: &CowStr<'_>, span: Range<usize>| {
        // For diagnostics, we want to underline the link's definition but `span` will point at
        // where the link is used. This is a problem for reference-style links, where the definition
        // is separate from the usage.

        match link {
            // `Borrowed` variant means the string (the link's destination) may come directly from
            // the markdown text and we can locate the original link destination.
            // NOTE: LinkReplacer also provides `Borrowed` but possibly from other sources,
            // so `locate()` can fall back to use `span`.
            CowStr::Borrowed(s) => locate(s, span),

            // For anything else, we can only use the provided range.
            CowStr::Boxed(_) | CowStr::Inlined(_) => MarkdownLinkRange::WholeLink(span),
        }
    };

    let span_for_offset_backward = |span: Range<usize>, open: u8, close: u8| {
        let mut open_brace = !0;
        let mut close_brace = !0;
        for (i, b) in md.as_bytes()[span.clone()].iter().copied().enumerate().rev() {
            let i = i + span.start;
            if b == close {
                close_brace = i;
                break;
            }
        }
        if close_brace < span.start || close_brace >= span.end {
            return MarkdownLinkRange::WholeLink(span);
        }
        let mut nesting = 1;
        for (i, b) in md.as_bytes()[span.start..close_brace].iter().copied().enumerate().rev() {
            let i = i + span.start;
            if b == close {
                nesting += 1;
            }
            if b == open {
                nesting -= 1;
            }
            if nesting == 0 {
                open_brace = i;
                break;
            }
        }
        assert!(open_brace != close_brace);
        if open_brace < span.start || open_brace >= span.end {
            return MarkdownLinkRange::WholeLink(span);
        }
        // do not actually include braces in the span
        let range = (open_brace + 1)..close_brace;
        MarkdownLinkRange::Destination(range.clone())
    };

    let span_for_offset_forward = |span: Range<usize>, open: u8, close: u8| {
        let mut open_brace = !0;
        let mut close_brace = !0;
        for (i, b) in md.as_bytes()[span.clone()].iter().copied().enumerate() {
            let i = i + span.start;
            if b == open {
                open_brace = i;
                break;
            }
        }
        if open_brace < span.start || open_brace >= span.end {
            return MarkdownLinkRange::WholeLink(span);
        }
        let mut nesting = 0;
        for (i, b) in md.as_bytes()[open_brace..span.end].iter().copied().enumerate() {
            let i = i + open_brace;
            if b == close {
                nesting -= 1;
            }
            if b == open {
                nesting += 1;
            }
            if nesting == 0 {
                close_brace = i;
                break;
            }
        }
        assert!(open_brace != close_brace);
        if open_brace < span.start || open_brace >= span.end {
            return MarkdownLinkRange::WholeLink(span);
        }
        // do not actually include braces in the span
        let range = (open_brace + 1)..close_brace;
        MarkdownLinkRange::Destination(range.clone())
    };

    Parser::new_with_broken_link_callback(
        md,
        main_body_opts(),
        Some(&mut |link: BrokenLink<'_>| Some((link.reference, "".into()))),
    )
        .into_offset_iter()
        .filter_map(|(event, span)| match event {
            Event::Start(Tag::Link(link_type, dest, _)) if may_be_doc_link(link_type) => {
                let range = match link_type {
                    // Link is pulled from the link itself.
                    LinkType::ReferenceUnknown | LinkType::ShortcutUnknown => {
                        span_for_offset_backward(span, b'[', b']')
                    }
                    LinkType::CollapsedUnknown => span_for_offset_forward(span, b'[', b']'),
                    LinkType::Inline => span_for_offset_backward(span, b'(', b')'),
                    // Link is pulled from elsewhere in the document.
                    LinkType::Reference | LinkType::Collapsed | LinkType::Shortcut => {
                        span_for_link(&dest, span)
                    }
                    LinkType::Autolink | LinkType::Email => unreachable!(),
                };
                preprocess_link(MarkdownLink { kind: link_type, range, link: dest.into_string() })
            }
            _ => None,
        })
        .collect()
}

//https://github.com/rust-lang/rust/blob/6dab6dc5fcd9655adfa7bfb3e59e5cae487184d2/src/librustdoc/html/markdown.rs#L1276