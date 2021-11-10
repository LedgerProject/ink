use ink_lang as ink;

#[ink::trait_definition]
pub trait TraitDefinition {
    #[ink(message, selector = 1, selector = "_")]
    fn message_1(&self);
}

fn main() {}
