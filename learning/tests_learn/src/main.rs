fn main() {
    println!("");
}

struct Fruits {
    bananas: u32, 
    apples: u32, 
}

impl Fruits {
    fn more_apples_than_bananas(&self) -> bool {
        if &self.apples > &self.bananas {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn more_apples() {
        let bucket = Fruits {
            bananas: 8, 
            apples: 7,
        };

        assert!(bucket.more_apples_than_bananas(), "I like bananananas");
    }
}