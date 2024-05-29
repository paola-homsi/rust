pub struct requestModel {
    name: String,
    actions: Vec<String>
}
impl requestModel {
    pub fn new(name: String) -> Self {
        Self { name, actions: Vec::new() }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn get_actions(&self) -> &Vec<String> {
        &self.actions
    }

    pub fn add_action(&mut self, action: String) {
        self.actions.push(action)
    }
}