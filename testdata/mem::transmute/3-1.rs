fn parse(mut self, grammar: impl Fn(&mut Self)) -> (Vec<Event>, Vec<SyntaxError>) {
  grammar(&mut self);
  for event in &self.events {
    assert!(event.is_some());
  }
  (unsafe { mem::transmute::<Vec<Option<Event>>, Vec<Event>>(self.events) }, self.errors)
 }
