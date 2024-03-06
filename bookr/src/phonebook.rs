pub mod contact;
use contact::Contact;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]

pub struct Phonebook(HashMap<String, Contact>);

impl Phonebook {
    pub fn new() -> Phonebook {
        Phonebook(HashMap::new())
    }

    pub fn add_contact(&mut self, name: &str, number: &str) {
        let contact = Contact::new(&name, &number);
        self.0.insert(contact.name.clone(), contact.clone());
    }

    pub fn get_contact(&self, name: &str) -> Option<&Contact> {
        if let Some(contact) = self.0.get(name) {
            Some(contact)
        } else {
            None
        }
    }

    pub fn edit_contact_name(&mut self, name: &str, new_name: &str) {
        if let Some(mut contact) = self.0.remove(name) {
            contact.edit_name(new_name);
            self.add_contact(&contact.name, &contact.phone);
        }
    }
    pub fn edit_contact_number(&mut self, name: &str, new_number: &str) {
        if let Some(mut contact) = self.0.remove(name) {
            contact.edit_phone(new_number);
            self.add_contact(&contact.name, &contact.phone);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_phonebook() {
        let phonebook = Phonebook::new();
        assert_eq!(phonebook, Phonebook(HashMap::new()));
    }
    #[test]
    fn test_add_contact() {
        let mut phonebook = Phonebook::new();
        phonebook.add_contact("Roger Waters", "905-555-9999");
        let contact = Contact::new("Roger Waters", "905-555-9999");
        assert_eq!(phonebook.0.get("Roger Waters"), Some(&contact));
    }

    #[test]
    fn test_get_contact() {
        let mut phonebook = Phonebook::new();
        phonebook.add_contact("Roger Waters", "905-555-9999");
        let contact = Contact::new("Roger Waters", "905-555-9999");
        assert_eq!(phonebook.get_contact("Roger Waters"), Some(&contact));
    }

    #[test]
    fn test_edit_contact_name() {
        let mut phonebook = Phonebook::new();
        phonebook.add_contact("Roger Waters", "905-555-9999");
        phonebook.edit_contact_name("Roger Waters", "David Gilmour");
        let updated_contact = Contact::new("David Gilmour", "905-555-9999");
        assert_eq!(
            phonebook.get_contact("David Gilmour"),
            Some(&updated_contact)
        );
    }

    #[test]
    fn test_edit_contact_number() {
        let mut phonebook = Phonebook::new();
        phonebook.add_contact("Roger Waters", "905-555-9999");
        phonebook.edit_contact_number("Roger Waters", "416-555-9999");
        let updated_contact = Contact::new("Roger Waters", "416-555-9999");
        assert_eq!(
            phonebook.get_contact("Roger Waters"),
            Some(&updated_contact)
        );
    }
}
