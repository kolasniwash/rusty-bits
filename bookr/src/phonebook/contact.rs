
#[derive(PartialEq, Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub phone: String
}

impl Contact {
    pub fn new(name: &str, phone: &str) -> Contact {
        Contact {
            name: name.to_string(),
            phone: phone.to_string()
        }
    }

    pub fn edit_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn edit_phone(&mut self, phone: &str) {
        self.phone = phone.to_string();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_contact() {
        let contact = Contact::new("Roger Waters", "905-555-9999");
        assert_eq!(contact.name, "Roger Waters");
        assert_eq!(contact.phone, "905-555-9999");
    }
    #[test]
    fn test_edit_name() {
        let mut contact = Contact::new("Roger Waters", "905-555-9999");
        contact.edit_name("David Gilmour");
        assert_eq!(contact.name, "David Gilmour");
        assert_eq!(contact.phone, "905-555-9999");
    }
    #[test]
    fn test_edit_phone() {
        let mut contact = Contact::new("Roger Waters", "905-555-9999");
        contact.edit_phone("416-555-9999");
        assert_eq!(contact.name, "Roger Waters");
        assert_eq!(contact.phone, "416-555-9999");
    }
}
