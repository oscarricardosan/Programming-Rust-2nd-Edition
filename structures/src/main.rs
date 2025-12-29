pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            println!("Reasigno jobs");
            std::mem::swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        // Método que toma propiedad de self, desinicializando las propiedades
        // older y younger al retornarlas.
        (self.older, self.younger)
    }
}

fn main() {
    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };

    q.push('0');
    q.push('1');

    //Lee la cola de older, esta vacía, voltea younger en older
    assert_eq!(q.pop(), Some('0'));

    q.push('8');
    assert_eq!(q.pop(), Some('1'));

    //Lee la cola de older, esta vacía por los dos anteriores pop, luego voltea younger en older
    assert_eq!(q.pop(), Some('8'));

    //Lee la cola de older, esta vacía al igual que younger, retorna None
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('9');
    assert!(!q.is_empty());

    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };
    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    // Ejecutar assert!(q.is_empty());
    // fallara, ya que self.older y self.younger
    // pasaron a ser propiedad de older y younger.
    println!("Finalización exitosa");
}
