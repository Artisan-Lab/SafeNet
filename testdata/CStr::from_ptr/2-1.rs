fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}: ", self.code())?;
    let msg = unsafe {
        CStr::from_ptr(tf::TF_Message(self.inner))
            .to_str()
            .unwrap_or("<invalid UTF-8 in message>")
    };
    f.write_str(msg)
}