#[macro_use]
extern crate hello_world;

use hello_world::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_this_later() {
        assert_eq!(token!(EOT), crate::TokenType::EOT);
    }
}
