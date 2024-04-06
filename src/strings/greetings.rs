pub struct Greetings {
    name: String,
}

impl Greetings {

    pub(crate) fn new (name: String) -> Self {
        Self { name }
    }

    pub(crate) fn hello(&self) -> String {
         "Hello there ".to_string() + &self.name
    }

}
